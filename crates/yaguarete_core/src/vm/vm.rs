use crate::error::YaguareteError;
use crate::value::Value;
use crate::vm::chunk::Chunk;
use crate::vm::compiler::CompiledFunction;
use crate::vm::opcode::OpCode;
use rustc_hash::FxHashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static RNG_STATE: AtomicU64 = AtomicU64::new(0);

fn next_random_u64() -> u64 {
    let current = RNG_STATE.load(Ordering::Relaxed);
    if current == 0 {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E37_79B9_7F4A_7C15);
        let pid = std::process::id() as u64;
        let seed = (nanos ^ (pid.wrapping_mul(0x517C_C1B7_2722_0A95))).max(1);
        let _ = RNG_STATE.compare_exchange(0, seed, Ordering::Relaxed, Ordering::Relaxed);
    }
    let prev = RNG_STATE.fetch_add(0x9E37_79B9_7F4A_7C15, Ordering::Relaxed);
    let mut z = prev.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn num_to_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Entero(n) => Some(*n as f64),
        Value::Real(r) => Some(*r),
        Value::Flotante(f) => Some(*f as f64),
        _ => None,
    }
}

fn parse_float_text(s: &str) -> Option<f64> {
    let lower = s.to_lowercase();
    if lower.contains("inf") || lower.contains("nan") {
        return None;
    }
    if !s.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    s.parse::<f64>().ok()
}

#[derive(Debug, Clone)]
struct CallFrame {
    chunk: Rc<Chunk>,
    ip: usize,
    slot_base: usize,
}

pub struct VM {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    globals: FxHashMap<String, Value>,
    functions: FxHashMap<String, Rc<CompiledFunction>>,
    struct_defs: FxHashMap<String, Vec<(String, crate::ast::DataType)>>,
    pub output: Vec<String>,
    current_line: usize,
    current_col: usize,
}

impl VM {
    pub fn new() -> Self {
        let mut globals = FxHashMap::default();
        globals.insert("PI".to_string(), Value::Real(std::f64::consts::PI));

        Self {
            stack: Vec::with_capacity(2048),
            frames: Vec::with_capacity(64),
            globals,
            functions: FxHashMap::default(),
            struct_defs: FxHashMap::default(),
            output: Vec::new(),
            current_line: 1,
            current_col: 1,
        }
    }

    pub fn run_program(
        &mut self,
        top_chunk: Chunk,
        functions: Vec<Rc<CompiledFunction>>,
        struct_defs: FxHashMap<String, Vec<(String, crate::ast::DataType)>>,
    ) -> Result<Value, YaguareteError> {
        self.struct_defs = struct_defs;
        for f in functions {
            self.functions.insert(f.name.to_uppercase(), f.clone());
        }

        let main_frame = CallFrame {
            chunk: Rc::new(top_chunk),
            ip: 0,
            slot_base: 0,
        };
        self.frames.push(main_frame);

        self.execute()
    }

    #[inline(always)]
    fn pop(&mut self) -> Result<Value, YaguareteError> {
        self.stack.pop().ok_or_else(|| YaguareteError::Runtime {
            message: "Subdesbordamiento de pila en la máquina virtual (stack underflow)".into(),
            line: self.current_line,
            column: self.current_col,
        })
    }

    #[inline(always)]
    fn peek(&self) -> Result<&Value, YaguareteError> {
        self.stack.last().ok_or_else(|| YaguareteError::Runtime {
            message: "Pila vacía al leer tope".into(),
            line: self.current_line,
            column: self.current_col,
        })
    }

    fn runtime_error(&self, msg: impl Into<String>) -> YaguareteError {
        YaguareteError::Runtime {
            message: msg.into(),
            line: self.current_line,
            column: self.current_col,
        }
    }

    fn instantiate_null_struct(&self, struct_name: &str) -> Value {
        let mut fields = std::collections::HashMap::new();
        let mut field_order = Vec::new();
        let found = self
            .struct_defs
            .get_key_value(struct_name)
            .or_else(|| {
                self.struct_defs
                    .iter()
                    .find(|(k, _)| k.eq_ignore_ascii_case(struct_name))
            });
        let canonical_name = if let Some((canon, field_defs)) = found {
            for (f_name, _) in field_defs {
                field_order.push(f_name.clone());
                fields.insert(f_name.clone(), Value::Nulo);
            }
            canon.clone()
        } else {
            struct_name.to_string()
        };
        let inst = crate::value::StructInstance {
            struct_name: canonical_name,
            fields,
            field_order,
        };
        Value::Struct(std::rc::Rc::new(std::cell::RefCell::new(inst)))
    }

    fn execute(&mut self) -> Result<Value, YaguareteError> {
        loop {
            let frame_idx = self.frames.len() - 1;
            let ip = self.frames[frame_idx].ip;
            let chunk = self.frames[frame_idx].chunk.clone();

            if ip >= chunk.code.len() {
                return Ok(Value::Nulo);
            }

            let byte = chunk.code[ip];
            self.frames[frame_idx].ip += 1;

            let op = match OpCode::from_u8(byte) {
                Some(o) => o,
                None => {
                    return Err(self.runtime_error(format!("OpCode inválido en bytecode: 0x{:02x}", byte)));
                }
            };

            match op {
                OpCode::Constant => {
                    let idx = chunk.read_u16(self.frames[frame_idx].ip);
                    self.frames[frame_idx].ip += 2;
                    let val = chunk.constants[idx as usize].clone();
                    self.stack.push(val);
                }
                OpCode::Nil => self.stack.push(Value::Nulo),
                OpCode::True => self.stack.push(Value::Booleano(true)),
                OpCode::False => self.stack.push(Value::Booleano(false)),
                OpCode::Int0 => self.stack.push(Value::Entero(0)),
                OpCode::Int1 => self.stack.push(Value::Entero(1)),
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Dup => {
                    let top = self.peek()?.clone();
                    self.stack.push(top);
                }
                OpCode::GetLocal => {
                    let slot = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let base = self.frames[frame_idx].slot_base;
                    let val = self.stack[base + slot].clone();
                    self.stack.push(val);
                }
                OpCode::SetLocal => {
                    let slot = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let base = self.frames[frame_idx].slot_base;
                    let val = self.pop()?;
                    self.stack[base + slot] = val;
                }
                OpCode::IncLocal => {
                    let slot = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let base = self.frames[frame_idx].slot_base;
                    match &mut self.stack[base + slot] {
                        Value::Entero(n) => *n += 1,
                        Value::Real(r) => *r += 1.0,
                        Value::Flotante(f) => *f += 1.0,
                        _ => return Err(self.runtime_error("IncLocal solo soportado sobre tipos numéricos")),
                    }
                }
                OpCode::DecLocal => {
                    let slot = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let base = self.frames[frame_idx].slot_base;
                    match &mut self.stack[base + slot] {
                        Value::Entero(n) => *n -= 1,
                        Value::Real(r) => *r -= 1.0,
                        Value::Flotante(f) => *f -= 1.0,
                        _ => return Err(self.runtime_error("DecLocal solo soportado sobre tipos numéricos")),
                    }
                }
                OpCode::GetGlobal => {
                    let idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let name = match &chunk.constants[idx] {
                        Value::Cadena(s) => s.as_str(),
                        _ => "",
                    };
                    if let Some(val) = self.globals.get(name) {
                        self.stack.push(val.clone());
                    } else if let Some(func) = self.functions.get(&name.to_uppercase()) {
                        // Devolver marcador de función
                        self.stack.push(Value::Cadena(format!("__FUNC__:{}", func.name)));
                    } else {
                        return Err(self.runtime_error(format!("Variable global no definida: '{}'", name)));
                    }
                }
                OpCode::SetGlobal => {
                    let idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let name = match &chunk.constants[idx] {
                        Value::Cadena(s) => s.clone(),
                        _ => String::new(),
                    };
                    let val = self.pop()?;
                    self.globals.insert(name, val);
                }
                OpCode::DefineGlobal => {
                    let idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let name = match &chunk.constants[idx] {
                        Value::Cadena(s) => s.clone(),
                        _ => String::new(),
                    };
                    let val = self.pop()?;
                    self.globals.insert(name, val);
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => self.stack.push(Value::Entero(x + y)),
                        (Value::Real(x), Value::Real(y)) => self.stack.push(Value::Real(x + y)),
                        (Value::Entero(x), Value::Real(y)) => self.stack.push(Value::Real(*x as f64 + y)),
                        (Value::Real(x), Value::Entero(y)) => self.stack.push(Value::Real(x + *y as f64)),
                        (Value::Flotante(x), Value::Flotante(y)) => self.stack.push(Value::Flotante(x + y)),
                        (Value::Cadena(s), other) => {
                            self.stack.push(Value::Cadena(format!("{}{}", s, other.to_canonical_string())));
                        }
                        (other, Value::Cadena(s)) => {
                            self.stack.push(Value::Cadena(format!("{}{}", other.to_canonical_string(), s)));
                        }
                        (Value::Vector(v1), Value::Vector(v2)) => {
                            let mut combined = Vec::with_capacity(v1.len() + v2.len());
                            combined.extend_from_slice(v1);
                            combined.extend_from_slice(v2);
                            self.stack.push(Value::Vector(combined));
                        }
                        _ => return Err(self.runtime_error("Operador '+' no compatible con los operandos dados")),
                    }
                }
                OpCode::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => self.stack.push(Value::Entero(x - y)),
                        (Value::Real(x), Value::Real(y)) => self.stack.push(Value::Real(x - y)),
                        (Value::Entero(x), Value::Real(y)) => self.stack.push(Value::Real(*x as f64 - y)),
                        (Value::Real(x), Value::Entero(y)) => self.stack.push(Value::Real(x - *y as f64)),
                        (Value::Flotante(x), Value::Flotante(y)) => self.stack.push(Value::Flotante(x - y)),
                        _ => return Err(self.runtime_error("Operador '-' solo compatible con números")),
                    }
                }
                OpCode::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => self.stack.push(Value::Entero(x * y)),
                        (Value::Real(x), Value::Real(y)) => self.stack.push(Value::Real(x * y)),
                        (Value::Entero(x), Value::Real(y)) => self.stack.push(Value::Real(*x as f64 * y)),
                        (Value::Real(x), Value::Entero(y)) => self.stack.push(Value::Real(x * *y as f64)),
                        (Value::Flotante(x), Value::Flotante(y)) => self.stack.push(Value::Flotante(x * y)),
                        _ => return Err(self.runtime_error("Operador '*' solo compatible con números")),
                    }
                }
                OpCode::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let (x, y) = match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => (*x as f64, *y as f64),
                        (Value::Real(x), Value::Real(y)) => (*x, *y),
                        (Value::Entero(x), Value::Real(y)) => (*x as f64, *y),
                        (Value::Real(x), Value::Entero(y)) => (*x, *y as f64),
                        _ => return Err(self.runtime_error("Operador '/' solo compatible con números")),
                    };
                    if y == 0.0 {
                        return Err(self.runtime_error("División por cero."));
                    }
                    self.stack.push(Value::Real(x / y));
                }
                OpCode::FloorDiv => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let (x, y) = match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => (*x, *y),
                        (Value::Real(x), Value::Real(y)) => (x.floor() as i64, y.floor() as i64),
                        (Value::Entero(x), Value::Real(y)) => (*x, y.floor() as i64),
                        (Value::Real(x), Value::Entero(y)) => (x.floor() as i64, *y),
                        _ => return Err(self.runtime_error("Operador '//' requiere operandos numéricos")),
                    };
                    if y == 0 {
                        return Err(self.runtime_error("División por cero."));
                    }
                    self.stack.push(Value::Entero(x.div_euclid(y)));
                }
                OpCode::Mod => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let (x, y) = match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => (*x, *y),
                        _ => return Err(self.runtime_error("Operador '%' requiere operandos enteros")),
                    };
                    if y == 0 {
                        return Err(self.runtime_error("Módulo por cero."));
                    }
                    self.stack.push(Value::Entero(x.rem_euclid(y)));
                }
                OpCode::Pow => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let base = match a {
                        Value::Entero(n) => n as f64,
                        Value::Real(r) => r,
                        _ => return Err(self.runtime_error("Operador '^' requiere base numérica")),
                    };
                    let exp = match b {
                        Value::Entero(n) => n as f64,
                        Value::Real(r) => r,
                        _ => return Err(self.runtime_error("Operador '^' requiere exponente numérico")),
                    };
                    if base < 0.0 && exp.fract() != 0.0 {
                        return Err(self.runtime_error("No se puede elevar un número negativo a un exponente fraccionario real."));
                    }
                    if base == 0.0 && exp < 0.0 {
                        return Err(self.runtime_error("División por cero: no se puede elevar 0 a un exponente negativo."));
                    }
                    self.stack.push(Value::Real(base.powf(exp)));
                }
                OpCode::Negate => {
                    let a = self.pop()?;
                    match a {
                        Value::Entero(n) => self.stack.push(Value::Entero(-n)),
                        Value::Real(r) => self.stack.push(Value::Real(-r)),
                        Value::Flotante(f) => self.stack.push(Value::Flotante(-f)),
                        _ => return Err(self.runtime_error("Operador unario '-' requiere número")),
                    }
                }
                OpCode::Equal => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Booleano(a == b));
                }
                OpCode::NotEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(Value::Booleano(a != b));
                }
                OpCode::Less => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let res = match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => x < y,
                        (Value::Real(x), Value::Real(y)) => x < y,
                        (Value::Entero(x), Value::Real(y)) => (*x as f64) < *y,
                        (Value::Real(x), Value::Entero(y)) => *x < (*y as f64),
                        _ => return Err(self.runtime_error("Operador '<' requiere números")),
                    };
                    self.stack.push(Value::Booleano(res));
                }
                OpCode::Greater => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let res = match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => x > y,
                        (Value::Real(x), Value::Real(y)) => x > y,
                        (Value::Entero(x), Value::Real(y)) => (*x as f64) > *y,
                        (Value::Real(x), Value::Entero(y)) => *x > (*y as f64),
                        _ => return Err(self.runtime_error("Operador '>' requiere números")),
                    };
                    self.stack.push(Value::Booleano(res));
                }
                OpCode::LessEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let res = match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => x <= y,
                        (Value::Real(x), Value::Real(y)) => x <= y,
                        (Value::Entero(x), Value::Real(y)) => (*x as f64) <= *y,
                        (Value::Real(x), Value::Entero(y)) => *x <= (*y as f64),
                        _ => return Err(self.runtime_error("Operador '<=' requiere números")),
                    };
                    self.stack.push(Value::Booleano(res));
                }
                OpCode::GreaterEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let res = match (&a, &b) {
                        (Value::Entero(x), Value::Entero(y)) => x >= y,
                        (Value::Real(x), Value::Real(y)) => x >= y,
                        (Value::Entero(x), Value::Real(y)) => (*x as f64) >= *y,
                        (Value::Real(x), Value::Entero(y)) => *x >= (*y as f64),
                        _ => return Err(self.runtime_error("Operador '>=' requiere números")),
                    };
                    self.stack.push(Value::Booleano(res));
                }
                OpCode::Not => {
                    let val = self.pop()?;
                    self.stack.push(Value::Booleano(!val.is_truthy()));
                }
                OpCode::Jump => {
                    let target = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip = target;
                }
                OpCode::JumpIfFalse => {
                    let target = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let val = self.peek()?;
                    if !val.is_truthy() {
                        self.frames[frame_idx].ip = target;
                    }
                }
                OpCode::Loop => {
                    let target = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip = target;
                }
                OpCode::Call => {
                    let argc = chunk.code[self.frames[frame_idx].ip] as usize;
                    self.frames[frame_idx].ip += 1;

                    // La función debe estar en la pila antes de los argumentos
                    let fn_val = &self.stack[self.stack.len() - 1 - argc];
                    let fn_name = match fn_val {
                        Value::Cadena(s) if s.starts_with("__FUNC__:") => s.strip_prefix("__FUNC__:").unwrap(),
                        _ => return Err(self.runtime_error("Intento de llamar un elemento no función")),
                    };

                    let upper_name = fn_name.to_uppercase();
                    let func = self.functions.get(&upper_name).cloned().ok_or_else(|| {
                        self.runtime_error(format!("Función '{}' no encontrada", fn_name))
                    })?;

                    if func.arity != argc {
                        return Err(self.runtime_error(format!(
                            "La función '{}' esperaba {} argumentos pero recibió {}",
                            fn_name, func.arity, argc
                        )));
                    }

                    let slot_base = self.stack.len() - argc;
                    let next_frame = CallFrame {
                        chunk: Rc::new(func.chunk.clone()),
                        ip: 0,
                        slot_base,
                    };
                    self.frames.push(next_frame);
                }
                OpCode::Return => {
                    let finished_frame = self.frames.pop().unwrap();
                    let ret_val = if self.stack.len() > finished_frame.slot_base {
                        self.stack.pop().unwrap()
                    } else {
                        Value::Nulo
                    };
                    self.stack.truncate(finished_frame.slot_base);

                    if self.frames.is_empty() {
                        return Ok(ret_val);
                    }

                    // Quitar el identificador de función previo de la cima
                    if !self.stack.is_empty() {
                        if let Some(Value::Cadena(s)) = self.stack.last() {
                            if s.starts_with("__FUNC__:") {
                                self.stack.pop();
                            }
                        }
                    }
                    self.stack.push(ret_val);
                }
                OpCode::BuildVector => {
                    let count = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let start = self.stack.len() - count;
                    let elements: Vec<Value> = self.stack.drain(start..).collect();
                    self.stack.push(Value::Vector(elements));
                }
                OpCode::GetIndex => {
                    let idx_val = self.pop()?;
                    let target = self.pop()?;
                    let idx = match idx_val {
                        Value::Entero(n) => n as usize,
                        Value::Real(r) => r as usize,
                        Value::Flotante(f) => f as usize,
                        _ => return Err(self.runtime_error("Índice de vector debe ser entero")),
                    };
                    match target {
                        Value::Vector(v) => {
                            if idx >= v.len() {
                                return Err(self.runtime_error(format!("Índice fuera de rango: {} (tamaño: {})", idx, v.len())));
                            }
                            self.stack.push(v[idx].clone());
                        }
                        Value::Cadena(s) => {
                            if let Some(ch) = s.chars().nth(idx) {
                                self.stack.push(Value::Caracter(ch));
                            } else {
                                return Err(self.runtime_error(format!("Índice fuera de límites: {} (longitud: {})", idx, s.chars().count())));
                            }
                        }
                        _ => return Err(self.runtime_error("Acceso por índice sobre elemento no indexable")),
                    }
                }
                OpCode::GetIndexLocal => {
                    let slot = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let idx_val = self.pop()?;
                    let idx = match idx_val {
                        Value::Entero(n) => n as usize,
                        Value::Real(r) => r as usize,
                        Value::Flotante(f) => f as usize,
                        _ => return Err(self.runtime_error("Índice de vector debe ser entero")),
                    };
                    let base = self.frames[frame_idx].slot_base;
                    match &self.stack[base + slot] {
                        Value::Vector(v) => {
                            if idx >= v.len() {
                                return Err(self.runtime_error(format!("Índice fuera de rango: {} (tamaño: {})", idx, v.len())));
                            }
                            let elem = v[idx].clone();
                            self.stack.push(elem);
                        }
                        Value::Cadena(s) => {
                            if let Some(ch) = s.chars().nth(idx) {
                                self.stack.push(Value::Caracter(ch));
                            } else {
                                return Err(self.runtime_error(format!("Índice fuera de límites: {} (longitud: {})", idx, s.chars().count())));
                            }
                        }
                        _ => return Err(self.runtime_error("Acceso por índice sobre elemento no indexable")),
                    }
                }
                OpCode::GetIndexGlobal => {
                    let name_idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let name = match &chunk.constants[name_idx] {
                        Value::Cadena(s) => s.as_str(),
                        _ => "",
                    };
                    let idx_val = self.pop()?;
                    let idx = match idx_val {
                        Value::Entero(n) => n as usize,
                        Value::Real(r) => r as usize,
                        Value::Flotante(f) => f as usize,
                        _ => return Err(self.runtime_error("Índice de vector debe ser entero")),
                    };
                    if let Some(target) = self.globals.get(name) {
                        match target {
                            Value::Vector(v) => {
                                if idx >= v.len() {
                                    return Err(self.runtime_error(format!("Índice fuera de rango: {} (tamaño: {})", idx, v.len())));
                                }
                                let elem = v[idx].clone();
                                self.stack.push(elem);
                            }
                            Value::Cadena(s) => {
                                if let Some(ch) = s.chars().nth(idx) {
                                    self.stack.push(Value::Caracter(ch));
                                } else {
                                    return Err(self.runtime_error(format!("Índice fuera de límites: {} (longitud: {})", idx, s.chars().count())));
                                }
                            }
                            _ => return Err(self.runtime_error("Acceso por índice sobre elemento no indexable")),
                        }
                    } else {
                        return Err(self.runtime_error(format!("Variable global '{}' no encontrada", name)));
                    }
                }
                OpCode::SetIndex => {
                    let val = self.pop()?;
                    let idx_val = self.pop()?;
                    let mut target = self.pop()?;
                    let idx = match idx_val {
                        Value::Entero(n) => n as usize,
                        Value::Real(r) => r as usize,
                        Value::Flotante(f) => f as usize,
                        _ => return Err(self.runtime_error("Índice de vector debe ser entero")),
                    };
                    match &mut target {
                        Value::Vector(v) => {
                            if idx >= v.len() {
                                return Err(self.runtime_error(format!("Índice fuera de rango: {} (tamaño: {})", idx, v.len())));
                            }
                            v[idx] = val;
                        }
                        _ => return Err(self.runtime_error("Asignación por índice sobre elemento no vector")),
                    }
                }
                OpCode::SetIndexLocal => {
                    let slot = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let val = self.pop()?;
                    let idx_val = self.pop()?;
                    let idx = match idx_val {
                        Value::Entero(n) => n as usize,
                        Value::Real(r) => r as usize,
                        Value::Flotante(f) => f as usize,
                        _ => return Err(self.runtime_error("Índice de vector debe ser entero")),
                    };
                    let base = self.frames[frame_idx].slot_base;
                    let cur_line = self.current_line;
                    let cur_col = self.current_col;
                    match &mut self.stack[base + slot] {
                        Value::Vector(v) => {
                            if idx >= v.len() {
                                return Err(YaguareteError::Runtime {
                                    message: format!("Índice fuera de rango: {} (tamaño: {})", idx, v.len()),
                                    line: cur_line,
                                    column: cur_col,
                                });
                            }
                            v[idx] = val;
                        }
                        _ => return Err(self.runtime_error("Asignación por índice sobre elemento no vector")),
                    }
                }
                OpCode::SetIndexGlobal => {
                    let name_idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let name = match &chunk.constants[name_idx] {
                        Value::Cadena(s) => s.as_str(),
                        _ => "",
                    };
                    let val = self.pop()?;
                    let idx_val = self.pop()?;
                    let idx = match idx_val {
                        Value::Entero(n) => n as usize,
                        Value::Real(r) => r as usize,
                        Value::Flotante(f) => f as usize,
                        _ => return Err(self.runtime_error("Índice de vector debe ser entero")),
                    };
                    let cur_line = self.current_line;
                    let cur_col = self.current_col;
                    if let Some(target) = self.globals.get_mut(name) {
                        match target {
                            Value::Vector(v) => {
                                if idx >= v.len() {
                                    return Err(YaguareteError::Runtime {
                                        message: format!("Índice fuera de rango: {} (tamaño: {})", idx, v.len()),
                                        line: cur_line,
                                        column: cur_col,
                                    });
                                }
                                v[idx] = val;
                            }
                            _ => return Err(self.runtime_error("Asignación por índice sobre elemento no vector")),
                        }
                    } else {
                        return Err(self.runtime_error(format!("Variable global '{}' no encontrada", name)));
                    }
                }
                OpCode::Length => {
                    let target = self.pop()?;
                    match target {
                        Value::Vector(v) => self.stack.push(Value::Entero(v.len() as i64)),
                        Value::Cadena(s) => self.stack.push(Value::Entero(s.chars().count() as i64)),
                        _ => return Err(self.runtime_error("Propiedad '.largo' solo disponible en vectores o cadenas")),
                    }
                }
                OpCode::LengthLocal => {
                    let slot = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let base = self.frames[frame_idx].slot_base;
                    match &self.stack[base + slot] {
                        Value::Vector(v) => self.stack.push(Value::Entero(v.len() as i64)),
                        Value::Cadena(s) => self.stack.push(Value::Entero(s.chars().count() as i64)),
                        _ => return Err(self.runtime_error("Propiedad '.largo' solo disponible en vectores o cadenas")),
                    }
                }
                OpCode::LengthGlobal => {
                    let name_idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let name = match &chunk.constants[name_idx] {
                        Value::Cadena(s) => s.as_str(),
                        _ => "",
                    };
                    if let Some(target) = self.globals.get(name) {
                        match target {
                            Value::Vector(v) => self.stack.push(Value::Entero(v.len() as i64)),
                            Value::Cadena(s) => self.stack.push(Value::Entero(s.chars().count() as i64)),
                            _ => return Err(self.runtime_error("Propiedad '.largo' solo disponible en vectores o cadenas")),
                        }
                    } else {
                        return Err(self.runtime_error(format!("Variable global '{}' no encontrada", name)));
                    }
                }
                OpCode::GetMember => {
                    let mem_idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let member = match &chunk.constants[mem_idx] {
                        Value::Cadena(s) => s.as_str(),
                        _ => "",
                    };
                    let target = self.pop()?;
                    match target {
                        Value::Struct(s) => {
                            let b = s.borrow();
                            let found = b.fields.get(member)
                                .cloned()
                                .or_else(|| b.fields.iter().find(|(k, _)| k.eq_ignore_ascii_case(member)).map(|(_, v)| v.clone()));
                            if let Some(v) = found {
                                self.stack.push(v);
                            } else {
                                return Err(self.runtime_error(format!("La estructura '{}' no tiene el campo '{}'", b.struct_name, member)));
                            }
                        }
                        Value::Nulo => return Err(self.runtime_error(format!("Intento de acceder al campo '{}' en una referencia nula (NULO)", member))),
                        _ => return Err(self.runtime_error("Acceso a miembro sobre elemento no estructura")),
                    }
                }
                OpCode::SetMember => {
                    let mem_idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let member = match &chunk.constants[mem_idx] {
                        Value::Cadena(s) => s.as_str(),
                        _ => "",
                    };
                    let val = self.pop()?;
                    let target = self.pop()?;
                    match target {
                        Value::Struct(s) => {
                            let mut b = s.borrow_mut();
                            let found_key = b.fields.keys().find(|k| k.eq_ignore_ascii_case(member)).cloned();
                            if let Some(key) = found_key {
                                b.fields.insert(key, val);
                            } else {
                                return Err(self.runtime_error(format!("La estructura '{}' no tiene el campo '{}'", b.struct_name, member)));
                            }
                        }
                        Value::Nulo => return Err(self.runtime_error(format!("Intento de asignar el campo '{}' en una referencia nula (NULO)", member))),
                        _ => return Err(self.runtime_error("Asignación a miembro sobre elemento no estructura")),
                    }
                }
                OpCode::NewStruct => {
                    let idx = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    self.frames[frame_idx].ip += 2;
                    let struct_name = match &chunk.constants[idx] {
                        Value::Cadena(s) => s.as_str(),
                        _ => "",
                    };
                    let inst = self.instantiate_null_struct(struct_name);
                    self.stack.push(inst);
                }
                OpCode::Print => {
                    let argc = chunk.code[self.frames[frame_idx].ip] as usize;
                    self.frames[frame_idx].ip += 1;
                    let start = self.stack.len() - argc;
                    let pieces: Vec<String> = self.stack.drain(start..).map(|v| v.to_canonical_string()).collect();
                    let line = pieces.join(" ");
                    println!("{}", line);
                    self.output.push(line);
                }
                OpCode::Builtin => {
                    let id = chunk.code[self.frames[frame_idx].ip];
                    let argc = chunk.code[self.frames[frame_idx].ip + 1] as usize;
                    self.frames[frame_idx].ip += 2;
                    self.execute_builtin(id, argc)?;
                }
                OpCode::Location => {
                    let l = chunk.read_u16(self.frames[frame_idx].ip) as usize;
                    let c = chunk.read_u16(self.frames[frame_idx].ip + 2) as usize;
                    self.frames[frame_idx].ip += 4;
                    self.current_line = l;
                    self.current_col = c;
                }
            }
        }
    }

    fn to_f64_conv(&self, v: &Value) -> Result<f64, YaguareteError> {
        let r = match v {
            Value::Entero(n) => Some(*n as f64),
            Value::Real(r) => Some(*r),
            Value::Flotante(f) => Some(*f as f64),
            Value::Booleano(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::Cadena(s) => {
                let t = s.trim();
                if let Ok(n) = t.parse::<i64>() {
                    Some(n as f64)
                } else {
                    parse_float_text(t)
                }
            }
            _ => None,
        };
        r.ok_or_else(|| self.runtime_error(format!("No se puede convertir '{}' a número real", v.to_canonical_string())))
    }

    fn execute_builtin(&mut self, id: u8, argc: usize) -> Result<(), YaguareteError> {
        let start = self.stack.len() - argc;
        let args: Vec<Value> = self.stack.drain(start..).collect();

        match id {
            // BuiltinId::Abs = 1
            1 => {
                let n = match &args[0] {
                    Value::Entero(n) => Value::Entero(n.abs()),
                    Value::Real(r) => Value::Real(r.abs()),
                    Value::Flotante(f) => Value::Flotante(f.abs()),
                    _ => return Err(self.runtime_error("ABS requiere argumento numérico")),
                };
                self.stack.push(n);
            }
            // BuiltinId::RaizCuadrada = 2
            2 => {
                let num = match &args[0] {
                    Value::Entero(n) => *n as f64,
                    Value::Real(r) => *r,
                    Value::Flotante(f) => *f as f64,
                    _ => return Err(self.runtime_error("RAIZCUADRADA requiere número")),
                };
                if num < 0.0 {
                    return Err(self.runtime_error("No se puede calcular la raíz cuadrada de un número negativo."));
                }
                self.stack.push(Value::Real(num.sqrt()));
            }
            // BuiltinId::Potencia = 3
            3 => {
                let base = match &args[0] {
                    Value::Entero(n) => *n as f64,
                    Value::Real(r) => *r,
                    _ => return Err(self.runtime_error("POTENCIA requiere base numérica")),
                };
                let exp = match &args[1] {
                    Value::Entero(n) => *n as f64,
                    Value::Real(r) => *r,
                    _ => return Err(self.runtime_error("POTENCIA requiere exponente numérico")),
                };
                self.stack.push(Value::Real(base.powf(exp)));
            }
            // BuiltinId::Aleatorio = 4
            4 => {
                let min = match &args[0] {
                    Value::Entero(n) => *n,
                    _ => return Err(self.runtime_error("ALEATORIO requiere min entero")),
                };
                let max = match &args[1] {
                    Value::Entero(n) => *n,
                    _ => return Err(self.runtime_error("ALEATORIO requiere max entero")),
                };
                if min > max {
                    return Err(self.runtime_error("En ALEATORIO(min, max), 'min' no puede ser mayor que 'max'."));
                }
                if min == max {
                    self.stack.push(Value::Entero(min));
                    return Ok(());
                }
                let range = (max as i128 - min as i128 + 1) as u128;
                let rnd = (next_random_u64() as u128) % range;
                let res = (min as i128 + rnd as i128) as i64;
                self.stack.push(Value::Entero(res));
            }
            // BuiltinId::EsPar = 5
            5 => {
                let n = match &args[0] {
                    Value::Entero(n) => *n,
                    _ => return Err(self.runtime_error("ESPAR requiere entero")),
                };
                self.stack.push(Value::Booleano(n % 2 == 0));
            }
            // BuiltinId::EsImpar = 6
            6 => {
                let n = match &args[0] {
                    Value::Entero(n) => *n,
                    _ => return Err(self.runtime_error("ESIMPAR requiere entero")),
                };
                self.stack.push(Value::Booleano(n % 2 != 0));
            }
            // BuiltinId::Mod = 7
            7 => {
                let a = match &args[0] {
                    Value::Entero(n) => *n,
                    _ => return Err(self.runtime_error("MOD requiere enteros")),
                };
                let b = match &args[1] {
                    Value::Entero(n) => *n,
                    _ => return Err(self.runtime_error("MOD requiere enteros")),
                };
                if b == 0 {
                    return Err(self.runtime_error("Módulo por cero."));
                }
                self.stack.push(Value::Entero(a.rem_euclid(b)));
            }
            // BuiltinId::Leer = 8
            8 => {
                let prompt = if argc > 0 {
                    match &args[0] {
                        Value::Cadena(s) => s.clone(),
                        Value::Caracter(c) => c.to_string(),
                        _ => return Err(self.runtime_error("LEER requiere un mensaje de tipo CADENA")),
                    }
                } else {
                    String::new()
                };
                use std::io::{BufRead, Write};
                let mut out = std::io::stdout();
                let _ = write!(out, "{}", prompt);
                let _ = out.flush();
                let mut buf = String::new();
                let line = match std::io::stdin().lock().read_line(&mut buf) {
                    Ok(0) | Err(_) => String::new(),
                    Ok(_) => buf.trim_end_matches(&['\r', '\n'][..]).to_string(),
                };
                self.stack.push(crate::eval::parse_input_line(&line));
            }
            // BuiltinId::Mayusculas = 9
            9 => {
                let s = args[0].to_canonical_string();
                self.stack.push(Value::Cadena(s.to_uppercase()));
            }
            // BuiltinId::Minusculas = 10
            10 => {
                let s = args[0].to_canonical_string();
                self.stack.push(Value::Cadena(s.to_lowercase()));
            }
            // BuiltinId::Recortar = 11
            11 => {
                let s = args[0].to_canonical_string();
                self.stack.push(Value::Cadena(s.trim().to_string()));
            }
            // BuiltinId::DividirCadena = 12
            12 => {
                let s = args[0].to_canonical_string();
                let sep = args[1].to_canonical_string();
                if sep.is_empty() {
                    return Err(self.runtime_error("El separador no puede estar vacío."));
                }
                let parts: Vec<Value> = s.split(&sep).map(|p| Value::Cadena(p.to_string())).collect();
                self.stack.push(Value::Vector(parts));
            }
            // BuiltinId::AEntero = 13
            13 => {
                let val = &args[0];
                let n = match val {
                    Value::Entero(n) => *n,
                    Value::Real(r) => r.trunc() as i64,
                    Value::Flotante(f) => f.trunc() as i64,
                    Value::Booleano(b) => *b as i64,
                    Value::Caracter(c) => match c.to_digit(10) {
                        Some(d) => d as i64,
                        None => return Err(self.runtime_error(format!("No se puede convertir '{}' a ENTERO", val.to_canonical_string()))),
                    },
                    Value::Cadena(s) => {
                        let t = s.trim();
                        if let Ok(n) = t.parse::<i64>() {
                            n
                        } else if let Some(f) = parse_float_text(t) {
                            f.trunc() as i64
                        } else {
                            return Err(self.runtime_error(format!("No se puede convertir '{}' a ENTERO", val.to_canonical_string())));
                        }
                    }
                    _ => return Err(self.runtime_error(format!("No se puede convertir '{}' a ENTERO", val.to_canonical_string()))),
                };
                self.stack.push(Value::Entero(n));
            }
            // BuiltinId::AReal = 14
            14 => {
                let r = self.to_f64_conv(&args[0])?;
                self.stack.push(Value::Real(r));
            }
            // BuiltinId::AFlotante = 15
            15 => {
                let r = self.to_f64_conv(&args[0])?;
                self.stack.push(Value::Flotante(r as f32));
            }
            // BuiltinId::ACadena = 16
            16 => {
                self.stack.push(Value::Cadena(args[0].to_canonical_string()));
            }
            // BuiltinId::ABooleano = 17
            17 => {
                let val = &args[0];
                let b = match val {
                    Value::Booleano(b) => Some(*b),
                    Value::Entero(n) => Some(*n != 0),
                    Value::Real(r) => Some(*r != 0.0),
                    Value::Flotante(f) => Some(*f != 0.0),
                    Value::Cadena(s) => match s.trim().to_uppercase().as_str() {
                        "VERDADERO" | "TRUE" => Some(true),
                        "FALSO" | "FALSE" => Some(false),
                        _ => None,
                    },
                    _ => None,
                };
                match b {
                    Some(b) => self.stack.push(Value::Booleano(b)),
                    None => return Err(self.runtime_error(format!("No se puede convertir '{}' a BOOLEANO", val.to_canonical_string()))),
                }
            }
            // BuiltinId::Min = 18, BuiltinId::Max = 19
            18 | 19 => {
                let a = &args[0];
                let b = &args[1];
                let (x, y) = match (num_to_f64(a), num_to_f64(b)) {
                    (Some(x), Some(y)) => (x, y),
                    _ => return Err(self.runtime_error(format!("{} requiere argumentos numéricos", if id == 18 { "MIN" } else { "MAX" }))),
                };
                let is_min = id == 18;
                let res = match (a, b) {
                    (Value::Entero(p), Value::Entero(q)) => Value::Entero(if is_min { *p.min(q) } else { *p.max(q) }),
                    (Value::Flotante(p), Value::Flotante(q)) => Value::Flotante(if is_min { (*p).min(*q) } else { (*p).max(*q) }),
                    _ => Value::Real(if is_min { x.min(y) } else { x.max(y) }),
                };
                self.stack.push(res);
            }
            // BuiltinId::Redondear = 20
            20 => {
                let x = num_to_f64(&args[0]).ok_or_else(|| self.runtime_error("REDONDEAR requiere un argumento numérico"))?;
                if argc == 1 {
                    self.stack.push(Value::Entero(x.round() as i64));
                } else {
                    let d = match &args[1] {
                        Value::Entero(d) if *d >= 0 => *d,
                        _ => return Err(self.runtime_error("Los decimales de REDONDEAR deben ser un entero mayor o igual a 0")),
                    };
                    let f = 10f64.powi(d.min(300) as i32);
                    let r = (x * f).round() / f;
                    self.stack.push(Value::Real(if r.is_finite() { r } else { x }));
                }
            }
            // BuiltinId::AbrirArchivo = 21
            21 => {
                let path = match &args[0] {
                    Value::Cadena(s) => s.clone(),
                    _ => args[0].to_canonical_string(),
                };
                let mode = match &args[1] {
                    Value::Cadena(s) => s.clone(),
                    _ => args[1].to_canonical_string(),
                };
                let handle = crate::value::ArchivoHandle::new(&path, &mode).map_err(|e| self.runtime_error(e.to_string()))?;
                self.stack.push(Value::Archivo(std::rc::Rc::new(std::cell::RefCell::new(handle))));
            }
            // Marcador especial 255: Crear vector dinámico de tamaño n (con estructuras NULO o 0s)
            255 => {
                let size = match &args[0] {
                    Value::Entero(n) => (*n).max(0) as usize,
                    _ => 0,
                };
                let struct_name = if argc >= 2 {
                    match &args[1] {
                        Value::Cadena(s) => Some(s.as_str()),
                        _ => None,
                    }
                } else {
                    None
                };
                let mut v = Vec::with_capacity(size);
                for _ in 0..size {
                    if let Some(s_name) = struct_name {
                        v.push(self.instantiate_null_struct(s_name));
                    } else {
                        v.push(Value::Entero(0));
                    }
                }
                self.stack.push(Value::Vector(v));
            }
            _ => {
                self.stack.push(Value::Nulo);
            }
        }
        Ok(())
    }
}
