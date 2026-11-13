use crate::ast::*;
use crate::error::YaguareteError;
use crate::value::Value;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
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

#[derive(Debug, Clone)]
pub struct FunctionData {
    pub params: Vec<(String, DataType)>,
    pub body: Vec<Statement>,
    pub return_type: DataType,
}

#[derive(Debug, Clone)]
pub struct VariableEntry {
    pub val: Value,
    pub var_type: DataType,
}

#[derive(Debug, Clone)]
pub struct Scope {
    entries: FxHashMap<String, VariableEntry>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            entries: FxHashMap::default(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            entries: FxHashMap::with_capacity_and_hasher(cap, Default::default()),
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

pub fn check_type_compatibility(var_type: &DataType, val: &mut Value) -> Result<(), YaguareteError> {
    match (var_type, &mut *val) {
        (DataType::Entero, Value::Entero(_)) => Ok(()),
        (DataType::Entero, Value::Real(r)) => {
            *val = Value::Entero(*r as i64);
            Ok(())
        }
        (DataType::Entero, Value::Flotante(f)) => {
            *val = Value::Entero(*f as i64);
            Ok(())
        }
        (DataType::Real, Value::Real(_)) => Ok(()),
        (DataType::Real, Value::Entero(n)) => {
            *val = Value::Real(*n as f64);
            Ok(())
        }
        (DataType::Real, Value::Flotante(f)) => {
            *val = Value::Real(*f as f64);
            Ok(())
        }
        (DataType::Flotante, Value::Flotante(_)) => Ok(()),
        (DataType::Flotante, Value::Entero(n)) => {
            *val = Value::Flotante(*n as f32);
            Ok(())
        }
        (DataType::Flotante, Value::Real(r)) => {
            *val = Value::Flotante(*r as f32);
            Ok(())
        }
        (DataType::Cadena, Value::Cadena(_)) => Ok(()),
        (DataType::Caracter, Value::Caracter(_)) => Ok(()),
        (DataType::Caracter, Value::Cadena(s)) if s.chars().count() == 1 => {
            *val = Value::Caracter(s.chars().next().unwrap());
            Ok(())
        }
        (DataType::Booleano, Value::Booleano(_)) => Ok(()),
        (DataType::Archivo, Value::Archivo(_)) | (DataType::Archivo, Value::Nulo) => Ok(()),
        (DataType::Vector(inner), Value::Vector(vec)) => {
            if vec.is_empty() {
                return Ok(());
            }
            let matches_homogeneous = match (inner.as_ref(), &vec[0]) {
                (DataType::Entero, Value::Entero(_)) => true,
                (DataType::Real, Value::Real(_)) => true,
                (DataType::Flotante, Value::Flotante(_)) => true,
                (DataType::Cadena, Value::Cadena(_)) => true,
                (DataType::Caracter, Value::Caracter(_)) => true,
                (DataType::Booleano, Value::Booleano(_)) => true,
                _ => false,
            };
            if matches_homogeneous {
                return Ok(());
            }
            for elem in vec.iter_mut() {
                check_type_compatibility(inner, elem)?;
            }
            Ok(())
        }
        (DataType::Vector(_), Value::Nulo) => Ok(()),
        (DataType::Vector(inner), other) => {
            let mut elem = other.clone();
            check_type_compatibility(inner, &mut elem)?;
            *val = Value::Vector(vec![elem]);
            Ok(())
        }
        (DataType::Matriz(_), Value::Vector(_)) | (DataType::Matriz(_), Value::Nulo) => Ok(()),
        (DataType::Estructura(name), Value::Struct(s)) => {
            if s.borrow().struct_name.eq_ignore_ascii_case(name) {
                Ok(())
            } else {
                Err(YaguareteError::Runtime {
                    message: format!(
                        "Tipo incompatible: se esperaba estructura '{}', pero se obtuvo '{}'",
                        name, s.borrow().struct_name
                    ),
                    line: 0,
                    column: 0,
                })
            }
        }
        (DataType::Estructura(_), Value::Nulo) => Ok(()),
        (DataType::Nulo, Value::Nulo) => Ok(()),
        (DataType::Void, _) => Ok(()),
        _ => Err(YaguareteError::Runtime {
            message: format!(
                "Incompatibilidad de tipos: no se puede asignar valor a variable de tipo '{}'",
                var_type
            ),
            line: 0,
            column: 0,
        }),
    }
}

#[derive(Debug, Clone)]
pub struct StructDefData {
    pub name: String,
    pub fields: Vec<(String, DataType)>,
}

/// Mensaje del error con el que el intérprete aborta cuando el depurador lo ordena.
pub const DEBUG_ABORT_MESSAGE: &str = "Ejecución detenida por el depurador.";

/// Decisión del depurador tras ser notificado de una sentencia.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugAction {
    Continue,
    Abort,
}

/// Hook invocado antes de ejecutar cada sentencia con línea conocida.
/// La implementación puede bloquear el hilo (p. ej. esperando un comando del IDE).
pub trait Debugger {
    fn on_statement(&mut self, line: usize, interp: &Interpreter) -> DebugAction;
}

/// Marco de pila visible desde el depurador (el primero es el más interno).
#[derive(Debug, Clone)]
pub struct DebugFrame {
    pub name: String,
    pub line: usize,
    pub variables: Vec<(String, DataType, Value)>,
}

struct CallFrame {
    name: String,
    call_line: usize,
    call_column: usize,
    scope_base: usize,
}

pub fn parse_input_line(text: &str) -> Value {
    let t = text.trim();
    let up = t.to_uppercase();
    if up == "VERDADERO" || up == "TRUE" {
        return Value::Booleano(true);
    }
    if up == "FALSO" || up == "FALSE" {
        return Value::Booleano(false);
    }
    if t.parse::<i64>().is_ok() {
        return Value::Entero(t.parse::<i64>().unwrap());
    }
    if t.contains('.') || t.contains('e') || t.contains('E') {
        let lower = t.to_lowercase();
        let looks_numeric = t.chars().any(|c| c.is_ascii_digit())
            && !lower.contains("inf")
            && !lower.contains("nan");
        if looks_numeric {
            if let Ok(r) = t.parse::<f64>() {
                return Value::Real(r);
            }
        }
    }
    Value::Cadena(text.to_string())
}

pub struct Interpreter {
    scopes: Vec<Scope>,
    scope_pool: Vec<Scope>,
    functions: FxHashMap<String, std::rc::Rc<FunctionData>>,
    struct_defs: FxHashMap<String, StructDefData>,
    pub output: Vec<String>,
    call_stack: Vec<CallFrame>,
    current_line: usize,
    current_column: usize,
    debugger: Option<Box<dyn Debugger>>,
    print_sink: Option<Box<dyn FnMut(&str)>>,
    input_reader: Option<Box<dyn FnMut(&str) -> Option<String>>>,
    input_disabled: bool,
}

#[derive(Debug)]
enum ControlFlow {
    None,
    Return(Value),
}

impl Interpreter {
    pub fn new() -> Self {
        let mut initial_scope = Scope::new();
        initial_scope.entries.insert(
            "PI".to_string(),
            VariableEntry {
                val: Value::Real(std::f64::consts::PI),
                var_type: DataType::Real,
            },
        );

        Self {
            scopes: vec![initial_scope],
            scope_pool: Vec::with_capacity(16),
            functions: FxHashMap::default(),
            struct_defs: FxHashMap::default(),
            output: Vec::new(),
            call_stack: Vec::new(),
            current_line: 0,
            current_column: 0,
            debugger: None,
            print_sink: None,
            input_reader: None,
            input_disabled: false,
        }
    }

    /// Retorna la línea actual de ejecución (1-based, 0 si no ha iniciado).
    pub fn current_line(&self) -> usize {
        self.current_line
    }

    /// Retorna la columna actual de ejecución (1-based, 0 si no ha iniciado).
    pub fn current_column(&self) -> usize {
        self.current_column
    }

    /// Asigna la línea y columna actual si el error devuelto no tiene ubicación asignada (line == 0).
    pub fn with_current_location(&self, err: YaguareteError) -> YaguareteError {
        match err {
            YaguareteError::Runtime { message, line, column } if line == 0 => YaguareteError::Runtime {
                message,
                line: self.current_line,
                column: if column == 0 { self.current_column } else { column },
            },
            other => other,
        }
    }

    /// Construye un YaguareteError::Runtime asociado a la línea y columna actuales del intérprete.
    pub fn runtime_error(&self, message: impl Into<String>) -> YaguareteError {
        YaguareteError::Runtime {
            message: message.into(),
            line: self.current_line,
            column: self.current_column,
        }
    }

    /// Instala un depurador que se consulta antes de cada sentencia.
    pub fn set_debugger(&mut self, debugger: Box<dyn Debugger>) {
        self.debugger = Some(debugger);
    }

    /// Redirige IMPRIMIR a un destino propio (en vez de stdout), p. ej. eventos DAP.
    pub fn set_print_sink(&mut self, sink: Box<dyn FnMut(&str)>) {
        self.print_sink = Some(sink);
    }

    /// Instala un lector de entrada para LEER (recibe el prompt; devuelve la línea sin salto o None en EOF).
    pub fn set_input_reader(&mut self, reader: Box<dyn FnMut(&str) -> Option<String>>) {
        self.input_reader = Some(reader);
        self.input_disabled = false;
    }

    /// Deshabilita LEER (p. ej. en DAP, donde stdin es el canal del protocolo).
    pub fn disable_input(&mut self) {
        self.input_disabled = true;
    }

    /// Profundidad actual de llamadas a funciones de usuario (0 = programa principal).
    pub fn debug_depth(&self) -> usize {
        self.call_stack.len()
    }

    /// Captura la pila de llamadas con las variables de cada marco (más interno primero).
    pub fn debug_snapshot(&self) -> Vec<DebugFrame> {
        let total = self.scopes.len();
        let collect = |from: usize, to: usize| -> Vec<(String, DataType, Value)> {
            let to = to.min(total);
            let from = from.min(to);
            let mut vars = Vec::new();
            for scope in &self.scopes[from..to] {
                let mut keys: Vec<&String> = scope.entries.keys().collect();
                keys.sort();
                for k in keys {
                    let entry = &scope.entries[k];
                    vars.push((k.clone(), entry.var_type.clone(), entry.val.clone()));
                }
            }
            vars
        };
        let n = self.call_stack.len();
        let mut frames = Vec::with_capacity(n + 1);
        for i in (0..=n).rev() {
            let (name, from) = if i == 0 {
                ("<principal>".to_string(), 0)
            } else {
                let f = &self.call_stack[i - 1];
                (f.name.clone(), f.scope_base)
            };
            let to = if i == n { total } else { self.call_stack[i].scope_base };
            let line = if i == n { self.current_line } else { self.call_stack[i].call_line };
            frames.push(DebugFrame {
                name,
                line,
                variables: collect(from, to),
            });
        }
        frames
    }

    fn push_scope(&mut self) {
        if let Some(mut recycled) = self.scope_pool.pop() {
            recycled.clear();
            self.scopes.push(recycled);
        } else {
            self.scopes.push(Scope::with_capacity(8));
        }
    }

    fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            let mut s = self.scopes.pop().unwrap();
            s.clear();
            if self.scope_pool.len() < 16 {
                self.scope_pool.push(s);
            }
        }
    }

    fn set_variable(&mut self, name: &str, mut val: Value) -> Result<(), YaguareteError> {
        if name.eq_ignore_ascii_case("PI") {
            return Err(self.runtime_error("No se puede reasignar la constante matemática 'PI'"));
        }
        let cur_line = self.current_line;
        let cur_col = self.current_column;
        // Buscar de adentro hacia afuera si ya existe
        for scope in self.scopes.iter_mut().rev() {
            if let Some(entry) = scope.entries.get_mut(name) {
                check_type_compatibility(&entry.var_type, &mut val)
                    .map_err(|e| match e {
                        YaguareteError::Runtime { message, line, column: _ } if line == 0 => {
                            YaguareteError::Runtime { message, line: cur_line, column: cur_col }
                        }
                        other => other,
                    })?;
                entry.val = val;
                return Ok(());
            }
        }
        // Si no existe, declarar en el scope actual
        self.scopes.last_mut().unwrap().entries.insert(
            name.to_string(),
            VariableEntry {
                val,
                var_type: DataType::Void,
            },
        );
        Ok(())
    }

    fn declare_variable(&mut self, name: &str, data_type: DataType, mut val: Value) -> Result<(), YaguareteError> {
        if name.eq_ignore_ascii_case("PI") {
            return Err(self.runtime_error("No se puede reasignar la constante matemática 'PI'"));
        }
        check_type_compatibility(&data_type, &mut val)
            .map_err(|e| self.with_current_location(e))?;
        let scope = self.scopes.last_mut().unwrap();
        scope.entries.insert(
            name.to_string(),
            VariableEntry {
                val,
                var_type: data_type,
            },
        );
        Ok(())
    }

    fn get_variable(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(entry) = scope.entries.get(name) {
                return Some(entry.val.clone());
            }
        }
        if name.eq_ignore_ascii_case("PI") {
            return Some(Value::Real(std::f64::consts::PI));
        }
        None
    }

    fn get_variable_mut<F>(&mut self, name: &str, f: F) -> Result<(), YaguareteError>
    where
        F: FnOnce(&mut Value) -> Result<(), YaguareteError>,
    {
        let cur_line = self.current_line;
        let cur_col = self.current_column;
        for scope in self.scopes.iter_mut().rev() {
            if let Some(entry) = scope.entries.get_mut(name) {
                return f(&mut entry.val).map_err(|e| match e {
                    YaguareteError::Runtime { message, line, column: _ } if line == 0 => {
                        YaguareteError::Runtime { message, line: cur_line, column: cur_col }
                    }
                    other => other,
                });
            }
        }
        Err(self.runtime_error(format!("Variable no definida: '{}'", name)))
    }

    fn find_archivo_in_expr(
        &mut self,
        expr: &Expression,
    ) -> Option<std::rc::Rc<std::cell::RefCell<crate::value::ArchivoHandle>>> {
        match expr {
            Expression::MemberAccess { target, .. } => {
                if let Ok(Value::Archivo(a)) = self.eval_expression(target) {
                    Some(a)
                } else {
                    None
                }
            }
            Expression::Unary { expr, .. } => self.find_archivo_in_expr(expr),
            Expression::Binary { left, right, .. } => {
                let l = self.find_archivo_in_expr(left);
                if l.is_some() {
                    l
                } else {
                    self.find_archivo_in_expr(right)
                }
            }
            _ => None,
        }
    }

    fn instantiate_struct(&self, name: &str, _parents: &mut std::collections::HashSet<String>) -> Value {
        if let Some(def) = self.struct_defs.get(&name.to_uppercase()) {
            let mut fields = HashMap::new();
            for (f_name, _) in &def.fields {
                fields.insert(f_name.clone(), Value::Nulo);
            }

            Value::Struct(std::rc::Rc::new(std::cell::RefCell::new(crate::value::StructInstance {
                struct_name: def.name.clone(),
                fields,
                field_order: def.fields.iter().map(|(n, _)| n.clone()).collect(),
            })))
        } else {
            Value::Nulo
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), YaguareteError> {
        // 1. Pre-registrar todas las funciones y estructuras declaradas
        for stmt in &program.statements {
            match stmt {
                Statement::FunctionDef {
                    name,
                    params,
                    return_type,
                    body,
                } => {
                    self.functions.insert(
                        name.to_uppercase(),
                        std::rc::Rc::new(FunctionData {
                            params: params.clone(),
                            body: body.clone(),
                            return_type: return_type.clone(),
                        }),
                    );
                }
                Statement::StructDef { name, fields } => {
                    let data = StructDefData {
                        name: name.clone(),
                        fields: fields.clone(),
                    };
                    self.struct_defs.insert(name.clone(), data.clone());
                    self.struct_defs.insert(name.to_uppercase(), data);
                }
                _ => {}
            }
        }

        // 2. Ejecutar sentencias secuencialmente
        for stmt in &program.statements {
            if let ControlFlow::Return(_) = self.execute_statement(stmt).map_err(|e| self.with_current_location(e))? {
                break;
            }
        }

        Ok(())
    }

    fn execute_statement(&mut self, stmt: &Statement) -> Result<ControlFlow, YaguareteError> {
        match stmt {
            Statement::VarDecl {
                var_type,
                name,
                size_exprs,
                init,
            } => {
                let initial_val = if let Some(init_expr) = init {
                    self.eval_expression(init_expr)?
                } else if !size_exprs.is_empty() {
                    let mut sizes = Vec::new();
                    for s_expr in size_exprs {
                        let size_val = self.eval_expression(s_expr)?;
                        let s = match size_val {
                            Value::Entero(n) => n.max(0) as usize,
                            Value::Real(r) => r.max(0.0) as usize,
                            Value::Flotante(f) => f.max(0.0) as usize,
                            _ => {
                                return Err(self.runtime_error("El tamaño debe ser un número entero"));
                            }
                        };
                        sizes.push(s);
                    }

                    fn get_default_elem(dt: &DataType, ctx: &mut Interpreter) -> Value {
                        match dt {
                            DataType::Entero => Value::Entero(0),
                            DataType::Real => Value::Real(0.0),
                            DataType::Flotante => Value::Flotante(0.0),
                            DataType::Cadena => Value::Cadena(String::new()),
                            DataType::Caracter => Value::Caracter(' '),
                            DataType::Booleano => Value::Booleano(false),
                            DataType::Estructura(s_name) => {
                                ctx.instantiate_struct(s_name, &mut std::collections::HashSet::new())
                            }
                            DataType::Vector(inner) | DataType::Matriz(inner) => get_default_elem(inner, ctx),
                            _ => Value::Nulo,
                        }
                    }

                    if sizes.len() == 1 {
                        let size = sizes[0];
                        let mut elements = Vec::with_capacity(size);
                        for _ in 0..size {
                            elements.push(get_default_elem(var_type, self));
                        }
                        Value::Vector(elements)
                    } else if sizes.len() == 2 {
                        let rows = sizes[0];
                        let cols = sizes[1];
                        let mut matrix = Vec::with_capacity(rows);
                        for _ in 0..rows {
                            let mut row = Vec::with_capacity(cols);
                            for _ in 0..cols {
                                row.push(get_default_elem(var_type, self));
                            }
                            matrix.push(Value::Vector(row));
                        }
                        Value::Vector(matrix)
                    } else {
                        Value::Vector(Vec::new())
                    }
                } else {
                    match var_type {
                        DataType::Entero => Value::Entero(0),
                        DataType::Real => Value::Real(0.0),
                        DataType::Flotante => Value::Flotante(0.0),
                        DataType::Cadena => Value::Cadena(String::new()),
                        DataType::Caracter => Value::Caracter(' '),
                        DataType::Booleano => Value::Booleano(false),
                        DataType::Vector(_) | DataType::Matriz(_) => Value::Vector(Vec::new()),
                        DataType::Estructura(s_name) => {
                            self.instantiate_struct(s_name, &mut std::collections::HashSet::new())
                        }
                        _ => Value::Nulo,
                    }
                };

                self.declare_variable(name, var_type.clone(), initial_val)?;
                Ok(ControlFlow::None)
            }
            Statement::Assign { target, val } => {
                if let Expression::Variable(name) = target {
                    if !name.eq_ignore_ascii_case("PI") {
                        let mut evaluated = self.eval_expression(val)?;
                        let cur_line = self.current_line;
                        let cur_col = self.current_column;
                        for scope in self.scopes.iter_mut().rev() {
                            if let Some(entry) = scope.entries.get_mut(name) {
                                check_type_compatibility(&entry.var_type, &mut evaluated)
                                    .map_err(|e| match e {
                                        YaguareteError::Runtime { message, line, column: _ } if line == 0 => {
                                            YaguareteError::Runtime { message, line: cur_line, column: cur_col }
                                        }
                                        other => other,
                                    })?;
                                entry.val = evaluated;
                                return Ok(ControlFlow::None);
                            }
                        }
                        // Si no existe, declarar en el scope actual
                        let scope = self.scopes.last_mut().unwrap();
                        scope.entries.insert(
                            name.to_string(),
                            VariableEntry {
                                val: evaluated,
                                var_type: DataType::Void,
                            },
                        );
                        return Ok(ControlFlow::None);
                    }
                }
                let evaluated = self.eval_expression(val)?;
                self.assign_to_target(target, evaluated)?;
                Ok(ControlFlow::None)
            }
            Statement::AssignOp { target, op, val } => {
                if let Expression::Variable(name) = target {
                    if !name.eq_ignore_ascii_case("PI") {
                        let right = self.eval_expression(val)?;
                        let mut handled = false;
                        for scope in self.scopes.iter_mut().rev() {
                            if let Some(entry) = scope.entries.get_mut(name) {
                                match (&mut entry.val, op, &right) {
                                    (Value::Entero(a), BinaryOp::Suma, Value::Entero(b)) => {
                                        *a += b;
                                        handled = true;
                                    }
                                    (Value::Entero(a), BinaryOp::Resta, Value::Entero(b)) => {
                                        *a -= b;
                                        handled = true;
                                    }
                                    (Value::Real(a), BinaryOp::Suma, Value::Real(b)) => {
                                        *a += b;
                                        handled = true;
                                    }
                                    (Value::Real(a), BinaryOp::Resta, Value::Real(b)) => {
                                        *a -= b;
                                        handled = true;
                                    }
                                    _ => {}
                                }
                                if handled {
                                    return Ok(ControlFlow::None);
                                }
                                break;
                            }
                        }
                    }
                }
                let right = self.eval_expression(val)?;
                let current = self.eval_expression(target)?;
                let res = self.eval_binary_op(&current, *op, &right)?;
                self.assign_to_target(target, res)?;
                Ok(ControlFlow::None)
            }
            Statement::Increment { target, is_inc } => {
                if let Expression::Variable(name) = target {
                    if !name.eq_ignore_ascii_case("PI") {
                        let mut handled = false;
                        for scope in self.scopes.iter_mut().rev() {
                            if let Some(entry) = scope.entries.get_mut(name) {
                                match &mut entry.val {
                                    Value::Entero(n) => {
                                        if *is_inc { *n += 1; } else { *n -= 1; }
                                        handled = true;
                                    }
                                    Value::Real(r) => {
                                        if *is_inc { *r += 1.0; } else { *r -= 1.0; }
                                        handled = true;
                                    }
                                    Value::Flotante(f) => {
                                        if *is_inc { *f += 1.0; } else { *f -= 1.0; }
                                        handled = true;
                                    }
                                    _ => {}
                                }
                                if handled {
                                    return Ok(ControlFlow::None);
                                }
                                break;
                            }
                        }
                    }
                }
                let current = self.eval_expression(target)?;
                let one = Value::Entero(1);
                let op = if *is_inc { BinaryOp::Suma } else { BinaryOp::Resta };
                let res = self.eval_binary_op(&current, op, &one)?;
                self.assign_to_target(target, res)?;
                Ok(ControlFlow::None)
            }
            Statement::If {
                condition,
                then_branch,
                else_if_branches,
                else_branch,
            } => {
                if self.eval_condition(condition)? {
                    for s in then_branch {
                        if let flow @ ControlFlow::Return(_) = self.execute_statement(s)? {
                            return Ok(flow);
                        }
                    }
                    return Ok(ControlFlow::None);
                }

                for (e_cond_expr, e_body) in else_if_branches {
                    if self.eval_condition(e_cond_expr)? {
                        for s in e_body {
                            if let flow @ ControlFlow::Return(_) = self.execute_statement(s)? {
                                return Ok(flow);
                            }
                        }
                        return Ok(ControlFlow::None);
                    }
                }

                if let Some(else_stmts) = else_branch {
                    for s in else_stmts {
                        if let flow @ ControlFlow::Return(_) = self.execute_statement(s)? {
                            return Ok(flow);
                        }
                    }
                }

                Ok(ControlFlow::None)
            }
            Statement::While { condition, body } => {
                let active_file = self.find_archivo_in_expr(condition);
                while self.eval_condition(condition)? {
                    for s in body {
                        if let flow @ ControlFlow::Return(_) = self.execute_statement(s)? {
                            return Ok(flow);
                        }
                    }
                    if let Some(ref file) = active_file {
                        file.borrow_mut().notify_loop_step();
                    }
                }
                Ok(ControlFlow::None)
            }
            Statement::DoWhile { body, condition } => {
                let active_file = self.find_archivo_in_expr(condition);
                loop {
                    for s in body {
                        if let flow @ ControlFlow::Return(_) = self.execute_statement(s)? {
                            return Ok(flow);
                        }
                    }
                    if let Some(ref file) = active_file {
                        file.borrow_mut().notify_loop_step();
                    }
                    if !self.eval_condition(condition)? {
                        break;
                    }
                }
                Ok(ControlFlow::None)
            }
            Statement::For {
                init,
                condition,
                update,
                body,
            } => {
                self.push_scope();
                self.execute_statement(init)?;

                while self.eval_condition(condition)? {
                    for s in body {
                        if let flow @ ControlFlow::Return(_) = self.execute_statement(s)? {
                            self.pop_scope();
                            return Ok(flow);
                        }
                    }
                    self.execute_statement(update)?;
                }

                self.pop_scope();
                Ok(ControlFlow::None)
            }
            Statement::Return(opt_expr) => {
                let val = if let Some(expr) = opt_expr {
                    self.eval_expression(expr)?
                } else {
                    Value::Nulo
                };
                Ok(ControlFlow::Return(val))
            }
            Statement::Print(args) => {
                let pieces: Result<Vec<String>, YaguareteError> = args
                    .iter()
                    .map(|arg| self.eval_expression(arg).map(|v| v.to_canonical_string()))
                    .collect();
                let text = pieces?.join(" ");
                if let Some(sink) = self.print_sink.as_mut() {
                    sink(&text);
                } else {
                    println!("{}", text);
                }
                self.output.push(text);
                Ok(ControlFlow::None)
            }
            Statement::Located { line, column, stmt } => {
                self.current_line = *line;
                self.current_column = *column;
                if self.debugger.is_some() {
                    if let Some(mut dbg) = self.debugger.take() {
                        let action = dbg.on_statement(*line, self);
                        self.debugger = Some(dbg);
                        if action == DebugAction::Abort {
                            return Err(YaguareteError::Runtime {
                                message: DEBUG_ABORT_MESSAGE.to_string(),
                                line: *line,
                                column: *column,
                            });
                        }
                    }
                }
                self.execute_statement(stmt)
                    .map_err(|e| self.with_current_location(e))
            }
            Statement::Expr(expr) => {
                self.eval_expression(expr)?;
                Ok(ControlFlow::None)
            }
            Statement::FunctionDef { .. } | Statement::StructDef { .. } => {
                // Ya pre-registradas
                Ok(ControlFlow::None)
            }
        }
    }

    fn assign_to_target(&mut self, target: &Expression, val: Value) -> Result<(), YaguareteError> {
        match target {
            Expression::Variable(name) => {
                self.set_variable(name, val)?;
                Ok(())
            }
            Expression::IndexAccess { target: inner, index } => {
                let idx = if let Expression::Variable(idx_name) = index.as_ref() {
                    let mut found_idx = None;
                    for scope in self.scopes.iter().rev() {
                        if let Some(entry) = scope.entries.get(idx_name) {
                            match &entry.val {
                                Value::Entero(n) => { found_idx = Some(*n as usize); break; }
                                Value::Real(r) => { found_idx = Some(*r as usize); break; }
                                Value::Flotante(f) => { found_idx = Some(*f as usize); break; }
                                _ => break,
                            }
                        }
                    }
                    if let Some(i) = found_idx {
                        i
                    } else {
                        let idx_val = self.eval_expression(index)?;
                        match idx_val {
                            Value::Entero(n) => n as usize,
                            Value::Real(r) => r as usize,
                            Value::Flotante(f) => f as usize,
                            _ => return Err(self.runtime_error("El índice de acceso debe ser un número entero")),
                        }
                    }
                } else {
                    let idx_val = self.eval_expression(index)?;
                    match idx_val {
                        Value::Entero(n) => n as usize,
                        Value::Real(r) => r as usize,
                        Value::Flotante(f) => f as usize,
                        _ => return Err(self.runtime_error("El índice de acceso debe ser un número entero")),
                    }
                };

                if let Expression::Variable(var_name) = inner.as_ref() {
                    return self.get_variable_mut(var_name, |container| match container {
                        Value::Vector(vec) => {
                            if idx >= vec.len() {
                                return Err(YaguareteError::Runtime {
                                    message: format!(
                                        "Índice fuera de rango: {} (tamaño: {})",
                                        idx,
                                        vec.len()
                                    ),
                                    line: 0,
                                    column: 0,
                                });
                            }
                            let promoted_val = match (&vec[idx], &val) {
                                (Value::Real(_), Value::Entero(n)) => Value::Real(*n as f64),
                                (Value::Flotante(_), Value::Entero(n)) => Value::Flotante(*n as f32),
                                (Value::Entero(_), Value::Real(r)) => Value::Entero(*r as i64),
                                _ => val,
                            };
                            vec[idx] = promoted_val;
                            Ok(())
                        }
                        _ => Err(YaguareteError::Runtime {
                            message: format!("La variable '{}' no es un vector indexable", var_name),
                            line: 0,
                            column: 0,
                        }),
                    });
                }

                if let Expression::IndexAccess {
                    target: root,
                    index: row_index,
                } = inner.as_ref()
                {
                    if let Expression::Variable(var_name) = root.as_ref() {
                        let row_idx_val = self.eval_expression(row_index)?;
                        let row_idx = match row_idx_val {
                            Value::Entero(n) => n as usize,
                            Value::Real(r) => r as usize,
                            Value::Flotante(f) => f as usize,
                            _ => {
                                return Err(self.runtime_error("El índice de fila debe ser un número entero"));
                            }
                        };

                        return self.get_variable_mut(var_name, |container| match container {
                            Value::Vector(matrix) => {
                                if row_idx >= matrix.len() {
                                    return Err(YaguareteError::Runtime {
                                        message: format!(
                                            "Índice de fila fuera de rango: {} (tamaño: {})",
                                            row_idx,
                                            matrix.len()
                                        ),
                                        line: 0,
                                        column: 0,
                                    });
                                }
                                match &mut matrix[row_idx] {
                                    Value::Vector(row) => {
                                        if idx >= row.len() {
                                            return Err(YaguareteError::Runtime {
                                                message: format!(
                                                    "Índice de columna fuera de rango: {} (tamaño: {})",
                                                    idx,
                                                    row.len()
                                                ),
                                                line: 0,
                                                column: 0,
                                            });
                                        }
                                        let promoted_val = match (&row[idx], &val) {
                                            (Value::Real(_), Value::Entero(n)) => Value::Real(*n as f64),
                                            (Value::Flotante(_), Value::Entero(n)) => Value::Flotante(*n as f32),
                                            (Value::Entero(_), Value::Real(r)) => Value::Entero(*r as i64),
                                            _ => val,
                                        };
                                        row[idx] = promoted_val;
                                        Ok(())
                                    }
                                    _ => Err(YaguareteError::Runtime {
                                        message: format!("La fila {} de '{}' no es un vector indexable", row_idx, var_name),
                                        line: 0,
                                        column: 0,
                                    }),
                                }
                            }
                            _ => Err(YaguareteError::Runtime {
                                message: format!("La variable '{}' no es una matriz", var_name),
                                line: 0,
                                column: 0,
                            }),
                        });
                    }
                }

                Err(self.runtime_error("Asignación compleja por índice aún no soportada"))
            }
            Expression::MemberAccess { target, member } => {
                let container = self.eval_expression(target)?;
                match container {
                    Value::Struct(inst) => {
                        let mut borrowed = inst.borrow_mut();
                        let mut found_key = None;
                        for k in borrowed.fields.keys() {
                            if k.eq_ignore_ascii_case(member) {
                                found_key = Some(k.clone());
                                break;
                            }
                        }
                        if let Some(key) = found_key {
                            borrowed.fields.insert(key, val);
                            Ok(())
                        } else {
                            let mut suggestion = String::new();
                            let s_name = borrowed.struct_name.clone();
                            let fields: Vec<String> = borrowed.fields.keys().cloned().collect();
                            for f in &fields {
                                if f.to_lowercase().starts_with(&member.to_lowercase())
                                    || member.to_lowercase().starts_with(&f.to_lowercase())
                                {
                                    suggestion = format!(". ¿Quisiste decir '{}'?", f);
                                    break;
                                }
                            }
                            Err(self.runtime_error(format!(
                                "La estructura '{}' no tiene el campo '{}'{}",
                                s_name, member, suggestion
                            )))
                        }
                    }
                    Value::Nulo => Err(self.runtime_error(format!("Intento de asignar el campo '{}' en una referencia nula (NULO)", member))),
                    _ => Err(self.runtime_error(format!("No se puede asignar el miembro '{}' sobre un tipo no estructura", member))),
                }
            }
            _ => Err(self.runtime_error("Destino de asignación no válido")),
        }
    }

    pub fn eval_expression(&mut self, expr: &Expression) -> Result<Value, YaguareteError> {
        match expr {
            Expression::LiteralInt(n) => Ok(Value::Entero(*n)),
            Expression::LiteralReal(r) => Ok(Value::Real(*r)),
            Expression::LiteralCadena(s) => Ok(Value::Cadena(s.clone())),
            Expression::LiteralCaracter(c) => Ok(Value::Caracter(*c)),
            Expression::LiteralBool(b) => Ok(Value::Booleano(*b)),
            Expression::LiteralNulo => Ok(Value::Nulo),
            Expression::LiteralVector(elements) => {
                let values: Result<Vec<Value>, YaguareteError> = elements
                    .iter()
                    .map(|elem| self.eval_expression(elem))
                    .collect();
                Ok(Value::Vector(values?))
            }
            Expression::Variable(name) => {
                if let Some(val) = self.get_variable(name) {
                    Ok(val)
                } else {
                    Err(self.runtime_error(format!("Variable no definida: '{}'", name)))
                }
            }
            Expression::Binary { left, op, right } => {
                match op {
                    BinaryOp::Y => {
                        let left_val = self.eval_expression(left)?;
                        if !left_val.is_truthy() {
                            return Ok(Value::Booleano(false));
                        }
                        let right_val = self.eval_expression(right)?;
                        Ok(Value::Booleano(right_val.is_truthy()))
                    }
                    BinaryOp::O => {
                        let left_val = self.eval_expression(left)?;
                        if left_val.is_truthy() {
                            return Ok(Value::Booleano(true));
                        }
                        let right_val = self.eval_expression(right)?;
                        Ok(Value::Booleano(right_val.is_truthy()))
                    }
                    _ => {
                        let left_val = self.eval_expression(left)?;
                        let right_val = self.eval_expression(right)?;
                        self.eval_binary_op(&left_val, *op, &right_val)
                    }
                }
            }
            Expression::Unary { op, expr } => {
                let val = self.eval_expression(expr)?;
                match op {
                    UnaryOp::No => Ok(Value::Booleano(!val.is_truthy())),
                    UnaryOp::Negativo => match val {
                        Value::Entero(n) => Ok(Value::Entero(-n)),
                        Value::Real(r) => Ok(Value::Real(-r)),
                        Value::Flotante(f) => Ok(Value::Flotante(-f)),
                        _ => Err(self.runtime_error("Operador unario '-' no aplicable a tipo no numérico")),
                    },
                    UnaryOp::Positivo => match val {
                        Value::Entero(n) => Ok(Value::Entero(n)),
                        Value::Real(r) => Ok(Value::Real(r)),
                        Value::Flotante(f) => Ok(Value::Flotante(f)),
                        _ => Err(self.runtime_error("Operador unario '+' no aplicable a tipo no numérico")),
                    },
                }
            }
            Expression::IndexAccess { target, index } => {
                let idx = if let Expression::Variable(idx_name) = index.as_ref() {
                    let mut found_idx = None;
                    for scope in self.scopes.iter().rev() {
                        if let Some(entry) = scope.entries.get(idx_name) {
                            match &entry.val {
                                Value::Entero(n) => { found_idx = Some(*n as usize); break; }
                                Value::Real(r) => { found_idx = Some(*r as usize); break; }
                                Value::Flotante(f) => { found_idx = Some(*f as usize); break; }
                                _ => break,
                            }
                        }
                    }
                    if let Some(i) = found_idx {
                        i
                    } else {
                        let index_val = self.eval_expression(index)?;
                        match index_val {
                            Value::Entero(n) => n as usize,
                            Value::Real(r) => r as usize,
                            Value::Flotante(f) => f as usize,
                            _ => return Err(self.runtime_error("El índice de vector debe ser un número entero")),
                        }
                    }
                } else {
                    let index_val = self.eval_expression(index)?;
                    match index_val {
                        Value::Entero(n) => n as usize,
                        Value::Real(r) => r as usize,
                        Value::Flotante(f) => f as usize,
                        _ => return Err(self.runtime_error("El índice de vector debe ser un número entero")),
                    }
                };

                // Optimización rápida: Si el target es una variable directa, leer sin clonar el vector completo
                if let Expression::Variable(var_name) = target.as_ref() {
                    for scope in self.scopes.iter().rev() {
                        if let Some(entry) = scope.entries.get(var_name) {
                            match &entry.val {
                                Value::Vector(vec) => {
                                    if idx >= vec.len() {
                                        return Err(self.runtime_error(format!(
                                            "Índice fuera de límites: {} (tamaño: {})",
                                            idx,
                                            vec.len()
                                        )));
                                    }
                                    return Ok(vec[idx].clone());
                                }
                                Value::Cadena(s) => {
                                    if let Some(ch) = s.chars().nth(idx) {
                                        return Ok(Value::Caracter(ch));
                                    } else {
                                        return Err(self.runtime_error(format!(
                                            "Índice fuera de límites: {} (longitud: {})",
                                            idx,
                                            s.chars().count()
                                        )));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                let target_val = self.eval_expression(target)?;
                match target_val {
                    Value::Vector(vec) => {
                        if idx >= vec.len() {
                            return Err(self.runtime_error(format!(
                                "Índice fuera de límites: {} (tamaño: {})",
                                idx,
                                vec.len()
                            )));
                        }
                        Ok(vec[idx].clone())
                    }
                    _ => Err(self.runtime_error("Acceso por índice sobre un elemento no indexable")),
                }
            }
            Expression::MemberAccess { target, member } => {
                if member == "largo" {
                    if let Expression::Variable(var_name) = target.as_ref() {
                        for scope in self.scopes.iter().rev() {
                            if let Some(entry) = scope.entries.get(var_name) {
                                match &entry.val {
                                    Value::Vector(v) => return Ok(Value::Entero(v.len() as i64)),
                                    Value::Cadena(s) => return Ok(Value::Entero(s.chars().count() as i64)),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                let target_val = self.eval_expression(target)?;
                if member == "largo" {
                    match target_val {
                        Value::Vector(v) => Ok(Value::Entero(v.len() as i64)),
                        Value::Cadena(s) => Ok(Value::Entero(s.chars().count() as i64)),
                        _ => Err(self.runtime_error("Propiedad '.largo' solo disponible en vectores o cadenas")),
                    }
                } else if member == "findearchivo" {
                    if let Value::Archivo(file) = target_val {
                        Ok(Value::Booleano(file.borrow().is_eof()))
                    } else {
                        Err(self.runtime_error("Propiedad '.findearchivo' solo disponible en tipo ARCHIVO"))
                    }
                } else {
                    match target_val {
                        Value::Struct(inst) => {
                            let borrowed = inst.borrow();
                            let mut found_val = None;
                            for (k, v) in &borrowed.fields {
                                if k.eq_ignore_ascii_case(member) {
                                    found_val = Some(v.clone());
                                    break;
                                }
                            }
                            if let Some(val) = found_val {
                                Ok(val)
                            } else {
                                let mut suggestion = String::new();
                                for k in borrowed.fields.keys() {
                                    if k.to_lowercase().starts_with(&member.to_lowercase())
                                        || member.to_lowercase().starts_with(&k.to_lowercase())
                                    {
                                        suggestion = format!(". ¿Quisiste decir '{}'?", k);
                                        break;
                                    }
                                }
                                Err(self.runtime_error(format!(
                                    "La estructura '{}' no tiene el campo '{}'{}",
                                    borrowed.struct_name, member, suggestion
                                )))
                            }
                        }
                        Value::Nulo => Err(self.runtime_error(format!("Intento de acceder al campo '{}' en una referencia nula (NULO)", member))),
                        _ => Err(self.runtime_error(format!("Acceso a miembro '.{}' sobre tipo no estructura", member))),
                    }
                }
            }
            Expression::MethodCall { target, method, args } => {
                let target_val = self.eval_expression(target)?;
                let method_upper = method.to_uppercase();

                match target_val {
                    Value::Archivo(file) => {
                        if method_upper == "LEER_LINEA" {
                            file.borrow_mut().read_line()
                        } else if method_upper == "ESCRIBIR_LINEA" {
                            let content = if !args.is_empty() {
                                self.eval_expression(&args[0])?
                            } else {
                                Value::Cadena(String::new())
                            };
                            file.borrow_mut().write_line(&content)?;
                            Ok(Value::Nulo)
                        } else if method_upper == "CERRAR_ARCHIVO" {
                            file.borrow_mut().close();
                            Ok(Value::Nulo)
                        } else {
                            Err(self.runtime_error(format!("Método '{}' no existe en tipo ARCHIVO", method)))
                        }
                    }
                    _ => Err(self.runtime_error(format!("Llamada a método '{}' sobre tipo no compatible", method))),
                }
            }
            Expression::Call { name, args } => {
                let upper = name.to_uppercase();

                // Built-in ABRIR_ARCHIVO
                if upper == "ABRIR_ARCHIVO" {
                    if args.len() != 2 {
                        return Err(self.runtime_error("ABRIR_ARCHIVO requiere exactamente 2 argumentos (ruta, modo)"));
                    }
                    let path_val = self.eval_expression(&args[0])?;
                    let mode_val = self.eval_expression(&args[1])?;
                    let path = match path_val {
                        Value::Cadena(s) => s,
                        _ => path_val.to_canonical_string(),
                    };
                    let mode = match mode_val {
                        Value::Cadena(s) => s,
                        _ => mode_val.to_canonical_string(),
                    };
                    let handle = crate::value::ArchivoHandle::new(&path, &mode)
                        .map_err(|e| self.with_current_location(e))?;
                    return Ok(Value::Archivo(std::rc::Rc::new(std::cell::RefCell::new(handle))));
                }

                // Built-in ESPAR / ESIMPAR
                if upper == "ESPAR" {
                    if args.len() != 1 {
                        return Err(self.runtime_error("ESPAR requiere exactamente 1 argumento"));
                    }
                    let val = self.eval_expression(&args[0])?;
                    let n = match val {
                        Value::Entero(n) => n,
                        Value::Real(r) => r as i64,
                        Value::Flotante(f) => f as i64,
                        _ => {
                            return Err(self.runtime_error("ESPAR requiere un número entero"));
                        }
                    };
                    return Ok(Value::Booleano(n % 2 == 0));
                }

                if upper == "ESIMPAR" {
                    if args.len() != 1 {
                        return Err(self.runtime_error("ESIMPAR requiere exactamente 1 argumento"));
                    }
                    let val = self.eval_expression(&args[0])?;
                    let n = match val {
                        Value::Entero(n) => n,
                        Value::Real(r) => r as i64,
                        Value::Flotante(f) => f as i64,
                        _ => {
                            return Err(self.runtime_error("ESIMPAR requiere un número entero"));
                        }
                    };
                    return Ok(Value::Booleano(n % 2 != 0));
                }

                if upper == "MOD" {
                    if args.len() != 2 {
                        return Err(self.runtime_error("MOD requiere exactamente 2 argumentos"));
                    }
                    let a_val = self.eval_expression(&args[0])?;
                    let b_val = self.eval_expression(&args[1])?;
                    let (a, b) = match (a_val, b_val) {
                        (Value::Entero(a), Value::Entero(b)) => (a, b),
                        _ => {
                            return Err(self.runtime_error("MOD requiere argumentos enteros"));
                        }
                    };
                    if b == 0 {
                        return Err(self.runtime_error("Módulo por cero."));
                    }
                    return Ok(Value::Entero(a.rem_euclid(b)));
                }

                if let Some(v) = self.eval_text_builtin(&upper, args)? {
                    return Ok(v);
                }

                // Built-in RAIZCUADRADA
                if upper == "RAIZCUADRADA" {
                    if args.len() != 1 {
                        return Err(self.runtime_error("RAIZCUADRADA requiere exactamente 1 argumento"));
                    }
                    let val = self.eval_expression(&args[0])?;
                    let num = match val {
                        Value::Entero(n) => n as f64,
                        Value::Real(r) => r,
                        Value::Flotante(f) => f as f64,
                        _ => {
                            return Err(self.runtime_error("RAIZCUADRADA requiere un argumento numérico"));
                        }
                    };
                    if num < 0.0 {
                        return Err(self.runtime_error("No se puede calcular la raíz cuadrada de un número negativo."));
                    }
                    return Ok(Value::Real(num.sqrt()));
                }

                // Built-in ABS
                if upper == "ABS" {
                    if args.len() != 1 {
                        return Err(self.runtime_error("ABS requiere exactamente 1 argumento"));
                    }
                    let val = self.eval_expression(&args[0])?;
                    return match val {
                        Value::Entero(n) => Ok(Value::Entero(n.abs())),
                        Value::Real(r) => Ok(Value::Real(r.abs())),
                        Value::Flotante(f) => Ok(Value::Flotante(f.abs())),
                        _ => Err(self.runtime_error("ABS requiere un argumento numérico")),
                    };
                }

                // Built-in POTENCIA
                if upper == "POTENCIA" {
                    if args.len() != 2 {
                        return Err(self.runtime_error("POTENCIA requiere exactamente 2 argumentos (base, exponente)"));
                    }
                    let base_val = self.eval_expression(&args[0])?;
                    let exp_val = self.eval_expression(&args[1])?;
                    let base = match base_val {
                        Value::Entero(n) => n as f64,
                        Value::Real(r) => r,
                        Value::Flotante(f) => f as f64,
                        _ => {
                            return Err(self.runtime_error("POTENCIA requiere argumentos numéricos"));
                        }
                    };
                    let exp = match exp_val {
                        Value::Entero(n) => n as f64,
                        Value::Real(r) => r,
                        Value::Flotante(f) => f as f64,
                        _ => {
                            return Err(self.runtime_error("POTENCIA requiere argumentos numéricos"));
                        }
                    };
                    if base < 0.0 && exp.fract() != 0.0 {
                        return Err(self.runtime_error("No se puede elevar un número negativo a un exponente fraccionario real."));
                    }
                    if base == 0.0 && exp < 0.0 {
                        return Err(self.runtime_error("División por cero: no se puede elevar 0 a un exponente negativo."));
                    }
                    return Ok(Value::Real(base.powf(exp)));
                }

                // Built-in ALEATORIO
                if upper == "ALEATORIO" {
                    if args.len() != 2 {
                        return Err(self.runtime_error("ALEATORIO requiere exactamente 2 argumentos (min, max)"));
                    }
                    let min_val = self.eval_expression(&args[0])?;
                    let max_val = self.eval_expression(&args[1])?;
                    let min = match min_val {
                        Value::Entero(n) => n,
                        Value::Real(r) => r as i64,
                        Value::Flotante(f) => f as i64,
                        _ => {
                            return Err(self.runtime_error("ALEATORIO requiere argumentos numéricos enteros"));
                        }
                    };
                    let max = match max_val {
                        Value::Entero(n) => n,
                        Value::Real(r) => r as i64,
                        Value::Flotante(f) => f as i64,
                        _ => {
                            return Err(self.runtime_error("ALEATORIO requiere argumentos numéricos enteros"));
                        }
                    };
                    if min > max {
                        return Err(self.runtime_error("En ALEATORIO(min, max), 'min' no puede ser mayor que 'max'."));
                    }
                    if min == max {
                        return Ok(Value::Entero(min));
                    }
                    let range = (max as i128 - min as i128 + 1) as u128;
                    let rnd = (next_random_u64() as u128) % range;
                    let result = (min as i128 + rnd as i128) as i64;
                    return Ok(Value::Entero(result));
                }

                // Función de usuario
                let func_data = if let Some(fd) = self.functions.get(&upper) {
                    fd.clone()
                } else {
                    return Err(self.runtime_error(format!("Función no definida: '{}'", name)));
                };

                if args.len() != func_data.params.len() {
                    return Err(self.runtime_error(format!(
                        "La función '{}' esperaba {} argumentos pero recibió {}",
                        name,
                        func_data.params.len(),
                        args.len()
                    )));
                }

                let mut evaluated_args = Vec::new();
                for arg in args {
                    evaluated_args.push(self.eval_expression(arg)?);
                }

                // Crear un nuevo frame de ejecución aislado para la función
                self.call_stack.push(CallFrame {
                    name: name.clone(),
                    call_line: self.current_line,
                    call_column: self.current_column,
                    scope_base: self.scopes.len(),
                });
                self.push_scope();
                for (i, (p_name, p_type)) in func_data.params.iter().enumerate() {
                    self.declare_variable(p_name, p_type.clone(), evaluated_args[i].clone())?;
                }

                let mut ret_val = Value::Nulo;
                for stmt in &func_data.body {
                    if let ControlFlow::Return(v) = self.execute_statement(stmt).map_err(|e| self.with_current_location(e))? {
                        ret_val = v;
                        break;
                    }
                }

                self.pop_scope();
                if let Some(f) = self.call_stack.pop() {
                    self.current_line = f.call_line;
                    self.current_column = f.call_column;
                }
                Ok(ret_val)
            }
        }
    }

    pub fn eval_condition(&mut self, expr: &Expression) -> Result<bool, YaguareteError> {
        match expr {
            Expression::LiteralBool(b) => Ok(*b),
            Expression::Unary { op: UnaryOp::No, expr } => {
                let inner = self.eval_condition(expr)?;
                Ok(!inner)
            }
            Expression::Binary { left, op, right } => {
                match op {
                    BinaryOp::Y => {
                        if !self.eval_condition(left)? {
                            return Ok(false);
                        }
                        self.eval_condition(right)
                    }
                    BinaryOp::O => {
                        if self.eval_condition(left)? {
                            return Ok(true);
                        }
                        self.eval_condition(right)
                    }
                    BinaryOp::Menor => {
                        // Fast-path 1: dos variables enteras directas (ej: j < size)
                        if let (Expression::Variable(l_name), Expression::Variable(r_name)) = (left.as_ref(), right.as_ref()) {
                            let mut l_int = None;
                            for scope in self.scopes.iter().rev() {
                                if let Some(entry) = scope.entries.get(l_name) {
                                    if let Value::Entero(n) = entry.val { l_int = Some(n); break; }
                                    if let Value::Real(r) = entry.val { l_int = Some(r as i64); break; }
                                }
                            }
                            let mut r_int = None;
                            for scope in self.scopes.iter().rev() {
                                if let Some(entry) = scope.entries.get(r_name) {
                                    if let Value::Entero(n) = entry.val { r_int = Some(n); break; }
                                    if let Value::Real(r) = entry.val { r_int = Some(r as i64); break; }
                                }
                            }
                            if let (Some(a), Some(b)) = (l_int, r_int) {
                                return Ok(a < b);
                            }
                        }

                        // Fast-path 2: dos accesos por índice al mismo vector (ej: a[j] < a[indice_menor])
                        if let (
                            Expression::IndexAccess { target: t1, index: i1 },
                            Expression::IndexAccess { target: t2, index: i2 },
                        ) = (left.as_ref(), right.as_ref()) {
                            if let (Expression::Variable(v1_name), Expression::Variable(v2_name)) = (t1.as_ref(), t2.as_ref()) {
                                if v1_name == v2_name {
                                    let idx1 = if let Expression::Variable(n1) = i1.as_ref() {
                                        let mut found = None;
                                        for scope in self.scopes.iter().rev() {
                                            if let Some(e) = scope.entries.get(n1) {
                                                if let Value::Entero(n) = e.val { found = Some(n as usize); break; }
                                            }
                                        }
                                        found
                                    } else { None };

                                    let idx2 = if let Expression::Variable(n2) = i2.as_ref() {
                                        let mut found = None;
                                        for scope in self.scopes.iter().rev() {
                                            if let Some(e) = scope.entries.get(n2) {
                                                if let Value::Entero(n) = e.val { found = Some(n as usize); break; }
                                            }
                                        }
                                        found
                                    } else { None };

                                    if let (Some(i1_val), Some(i2_val)) = (idx1, idx2) {
                                        for scope in self.scopes.iter().rev() {
                                            if let Some(entry) = scope.entries.get(v1_name) {
                                                if let Value::Vector(vec) = &entry.val {
                                                    if i1_val < vec.len() && i2_val < vec.len() {
                                                        match (&vec[i1_val], &vec[i2_val]) {
                                                            (Value::Entero(a), Value::Entero(b)) => return Ok(a < b),
                                                            (Value::Real(a), Value::Real(b)) => return Ok(a < b),
                                                            _ => break,
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        let l = self.eval_expression(left)?;
                        let r = self.eval_expression(right)?;
                        match (&l, &r) {
                            (Value::Entero(a), Value::Entero(b)) => Ok(a < b),
                            (Value::Real(a), Value::Real(b)) => Ok(a < b),
                            (Value::Entero(a), Value::Real(b)) => Ok((*a as f64) < *b),
                            (Value::Real(a), Value::Entero(b)) => Ok(*a < (*b as f64)),
                            _ => {
                                let res = self.eval_binary_op(&l, BinaryOp::Menor, &r)?;
                                Ok(res.is_truthy())
                            }
                        }
                    }
                    BinaryOp::Mayor => {
                        let l = self.eval_expression(left)?;
                        let r = self.eval_expression(right)?;
                        match (&l, &r) {
                            (Value::Entero(a), Value::Entero(b)) => Ok(a > b),
                            (Value::Real(a), Value::Real(b)) => Ok(a > b),
                            (Value::Entero(a), Value::Real(b)) => Ok((*a as f64) > *b),
                            (Value::Real(a), Value::Entero(b)) => Ok(*a > (*b as f64)),
                            _ => {
                                let res = self.eval_binary_op(&l, BinaryOp::Mayor, &r)?;
                                Ok(res.is_truthy())
                            }
                        }
                    }
                    BinaryOp::MenorIgual => {
                        let l = self.eval_expression(left)?;
                        let r = self.eval_expression(right)?;
                        match (&l, &r) {
                            (Value::Entero(a), Value::Entero(b)) => Ok(a <= b),
                            (Value::Real(a), Value::Real(b)) => Ok(a <= b),
                            (Value::Entero(a), Value::Real(b)) => Ok((*a as f64) <= *b),
                            (Value::Real(a), Value::Entero(b)) => Ok(*a <= (*b as f64)),
                            _ => {
                                let res = self.eval_binary_op(&l, BinaryOp::MenorIgual, &r)?;
                                Ok(res.is_truthy())
                            }
                        }
                    }
                    BinaryOp::MayorIgual => {
                        let l = self.eval_expression(left)?;
                        let r = self.eval_expression(right)?;
                        match (&l, &r) {
                            (Value::Entero(a), Value::Entero(b)) => Ok(a >= b),
                            (Value::Real(a), Value::Real(b)) => Ok(a >= b),
                            (Value::Entero(a), Value::Real(b)) => Ok((*a as f64) >= *b),
                            (Value::Real(a), Value::Entero(b)) => Ok(*a >= (*b as f64)),
                            _ => {
                                let res = self.eval_binary_op(&l, BinaryOp::MayorIgual, &r)?;
                                Ok(res.is_truthy())
                            }
                        }
                    }
                    BinaryOp::IgualQue => {
                        let l = self.eval_expression(left)?;
                        let r = self.eval_expression(right)?;
                        Ok(l == r)
                    }
                    BinaryOp::Distinto => {
                        // Fast-path: a[indice_menor] != a[i]
                        if let (
                            Expression::IndexAccess { target: t1, index: i1 },
                            Expression::IndexAccess { target: t2, index: i2 },
                        ) = (left.as_ref(), right.as_ref()) {
                            if let (Expression::Variable(v1_name), Expression::Variable(v2_name)) = (t1.as_ref(), t2.as_ref()) {
                                if v1_name == v2_name {
                                    let idx1 = if let Expression::Variable(n1) = i1.as_ref() {
                                        let mut found = None;
                                        for scope in self.scopes.iter().rev() {
                                            if let Some(e) = scope.entries.get(n1) {
                                                if let Value::Entero(n) = e.val { found = Some(n as usize); break; }
                                            }
                                        }
                                        found
                                    } else { None };

                                    let idx2 = if let Expression::Variable(n2) = i2.as_ref() {
                                        let mut found = None;
                                        for scope in self.scopes.iter().rev() {
                                            if let Some(e) = scope.entries.get(n2) {
                                                if let Value::Entero(n) = e.val { found = Some(n as usize); break; }
                                            }
                                        }
                                        found
                                    } else { None };

                                    if let (Some(i1_val), Some(i2_val)) = (idx1, idx2) {
                                        for scope in self.scopes.iter().rev() {
                                            if let Some(entry) = scope.entries.get(v1_name) {
                                                if let Value::Vector(vec) = &entry.val {
                                                    if i1_val < vec.len() && i2_val < vec.len() {
                                                        match (&vec[i1_val], &vec[i2_val]) {
                                                            (Value::Entero(a), Value::Entero(b)) => return Ok(a != b),
                                                            (Value::Real(a), Value::Real(b)) => return Ok(a != b),
                                                            _ => break,
                                                        }
                                                    }
                                                }
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let l = self.eval_expression(left)?;
                        let r = self.eval_expression(right)?;
                        Ok(l != r)
                    }
                    _ => {
                        let val = self.eval_expression(expr)?;
                        Ok(val.is_truthy())
                    }
                }
            }
            _ => {
                let val = self.eval_expression(expr)?;
                Ok(val.is_truthy())
            }
        }
    }

    fn eval_binary_op(&mut self, left: &Value, op: BinaryOp, right: &Value) -> Result<Value, YaguareteError> {
        match op {
            BinaryOp::Suma => {
                // Cadena + Cualquier cosa
                if let Value::Cadena(s) = left {
                    return Ok(Value::Cadena(format!("{}{}", s, right.to_canonical_string())));
                }
                if let Value::Cadena(s) = right {
                    return Ok(Value::Cadena(format!("{}{}", left.to_canonical_string(), s)));
                }

                // Vector + Vector (Concatenación)
                if let (Value::Vector(v1), Value::Vector(v2)) = (left, right) {
                    let mut combined = Vec::with_capacity(v1.len() + v2.len());
                    combined.extend_from_slice(v1);
                    combined.extend_from_slice(v2);
                    return Ok(Value::Vector(combined));
                }

                // Operaciones numéricas
                match (left, right) {
                    (Value::Entero(a), Value::Entero(b)) => Ok(Value::Entero(a + b)),
                    (Value::Real(a), Value::Real(b)) => Ok(Value::Real(a + b)),
                    (Value::Entero(a), Value::Real(b)) => Ok(Value::Real(*a as f64 + b)),
                    (Value::Real(a), Value::Entero(b)) => Ok(Value::Real(a + *b as f64)),
                    (Value::Flotante(a), Value::Flotante(b)) => Ok(Value::Flotante(a + b)),
                    _ => Err(self.runtime_error(format!("Operador '+' no compatible con tipos '{:?}' y '{:?}'", left, right))),
                }
            }
            BinaryOp::Resta => match (left, right) {
                (Value::Entero(a), Value::Entero(b)) => Ok(Value::Entero(a - b)),
                (Value::Real(a), Value::Real(b)) => Ok(Value::Real(a - b)),
                (Value::Entero(a), Value::Real(b)) => Ok(Value::Real(*a as f64 - b)),
                (Value::Real(a), Value::Entero(b)) => Ok(Value::Real(a - *b as f64)),
                (Value::Flotante(a), Value::Flotante(b)) => Ok(Value::Flotante(a - b)),
                _ => Err(self.runtime_error("Operador '-' solo compatible con números")),
            },
            BinaryOp::Mult => match (left, right) {
                (Value::Entero(a), Value::Entero(b)) => Ok(Value::Entero(a * b)),
                (Value::Real(a), Value::Real(b)) => Ok(Value::Real(a * b)),
                (Value::Entero(a), Value::Real(b)) => Ok(Value::Real(*a as f64 * b)),
                (Value::Real(a), Value::Entero(b)) => Ok(Value::Real(a * *b as f64)),
                (Value::Flotante(a), Value::Flotante(b)) => Ok(Value::Flotante(a * b)),
                _ => Err(self.runtime_error("Operador '*' solo compatible con números")),
            },
            BinaryOp::Div => {
                // División real '/': siempre retorna REAL
                let (a, b) = match (left, right) {
                    (Value::Entero(a), Value::Entero(b)) => (*a as f64, *b as f64),
                    (Value::Real(a), Value::Real(b)) => (*a, *b),
                    (Value::Entero(a), Value::Real(b)) => (*a as f64, *b),
                    (Value::Real(a), Value::Entero(b)) => (*a, *b as f64),
                    _ => {
                        return Err(self.runtime_error("Operador '/' solo compatible con números"));
                    }
                };
                if b == 0.0 {
                    return Err(self.runtime_error("División por cero."));
                }
                Ok(Value::Real(a / b))
            }
            BinaryOp::DivEntera => {
                // División entera '//': siempre retorna ENTERO
                let (a, b) = match (left, right) {
                    (Value::Entero(a), Value::Entero(b)) => (*a, *b),
                    (Value::Real(a), Value::Real(b)) => (a.floor() as i64, b.floor() as i64),
                    (Value::Entero(a), Value::Real(b)) => (*a, b.floor() as i64),
                    (Value::Real(a), Value::Entero(b)) => (a.floor() as i64, *b),
                    _ => {
                        return Err(self.runtime_error("Operador '//' requiere operandos numéricos"));
                    }
                };
                if b == 0 {
                    return Err(self.runtime_error("División por cero."));
                }
                Ok(Value::Entero(a.div_euclid(b)))
            }
            BinaryOp::Mod => {
                // Módulo '%': siempre retorna ENTERO
                let (a, b) = match (left, right) {
                    (Value::Entero(a), Value::Entero(b)) => (*a, *b),
                    _ => {
                        return Err(self.runtime_error("Operador '%' requiere operandos enteros"));
                    }
                };
                if b == 0 {
                    return Err(self.runtime_error("Módulo por cero."));
                }
                Ok(Value::Entero(a.rem_euclid(b)))
            }
            BinaryOp::Potencia => {
                let base = match left {
                    Value::Entero(n) => *n as f64,
                    Value::Real(r) => *r,
                    Value::Flotante(f) => *f as f64,
                    _ => {
                        return Err(self.runtime_error("Operador '^' solo compatible con números"));
                    }
                };
                let exp = match right {
                    Value::Entero(n) => *n as f64,
                    Value::Real(r) => *r,
                    Value::Flotante(f) => *f as f64,
                    _ => {
                        return Err(self.runtime_error("Operador '^' solo compatible con números"));
                    }
                };
                if base < 0.0 && exp.fract() != 0.0 {
                    return Err(self.runtime_error("No se puede elevar un número negativo a un exponente fraccionario real."));
                }
                if base == 0.0 && exp < 0.0 {
                    return Err(self.runtime_error("División por cero: no se puede elevar 0 a un exponente negativo."));
                }
                Ok(Value::Real(base.powf(exp)))
            }
            BinaryOp::IgualQue => Ok(Value::Booleano(left == right)),
            BinaryOp::Distinto => Ok(Value::Booleano(left != right)),
            BinaryOp::Menor => match (left, right) {
                (Value::Entero(a), Value::Entero(b)) => Ok(Value::Booleano(a < b)),
                (Value::Real(a), Value::Real(b)) => Ok(Value::Booleano(a < b)),
                (Value::Entero(a), Value::Real(b)) => Ok(Value::Booleano((*a as f64) < *b)),
                (Value::Real(a), Value::Entero(b)) => Ok(Value::Booleano(*a < (*b as f64))),
                _ => Err(self.runtime_error("Operador '<' requiere números")),
            },
            BinaryOp::Mayor => match (left, right) {
                (Value::Entero(a), Value::Entero(b)) => Ok(Value::Booleano(a > b)),
                (Value::Real(a), Value::Real(b)) => Ok(Value::Booleano(a > b)),
                (Value::Entero(a), Value::Real(b)) => Ok(Value::Booleano((*a as f64) > *b)),
                (Value::Real(a), Value::Entero(b)) => Ok(Value::Booleano(*a > (*b as f64))),
                _ => Err(self.runtime_error("Operador '>' requiere números")),
            },
            BinaryOp::MenorIgual => match (left, right) {
                (Value::Entero(a), Value::Entero(b)) => Ok(Value::Booleano(a <= b)),
                (Value::Real(a), Value::Real(b)) => Ok(Value::Booleano(a <= b)),
                (Value::Entero(a), Value::Real(b)) => Ok(Value::Booleano((*a as f64) <= *b)),
                (Value::Real(a), Value::Entero(b)) => Ok(Value::Booleano(*a <= (*b as f64))),
                _ => Err(self.runtime_error("Operador '<=' requiere números")),
            },
            BinaryOp::MayorIgual => match (left, right) {
                (Value::Entero(a), Value::Entero(b)) => Ok(Value::Booleano(a >= b)),
                (Value::Real(a), Value::Real(b)) => Ok(Value::Booleano(a >= b)),
                (Value::Entero(a), Value::Real(b)) => Ok(Value::Booleano((*a as f64) >= *b)),
                (Value::Real(a), Value::Entero(b)) => Ok(Value::Booleano(*a >= (*b as f64))),
                _ => Err(self.runtime_error("Operador '>=' requiere números")),
            },
            BinaryOp::Y => Ok(Value::Booleano(left.is_truthy() && right.is_truthy())),
            BinaryOp::O => Ok(Value::Booleano(left.is_truthy() || right.is_truthy())),
        }
    }

    pub fn eval_repl_input(&mut self, input: &str) -> Result<Option<Value>, YaguareteError> {
        let mut lexer = crate::lexer::Lexer::new(input);
        let tokens = lexer.tokenize()?;

        if tokens.is_empty()
            || (tokens.len() == 1 && matches!(tokens[0].token_type, crate::token::TokenType::Eof))
        {
            return Ok(None);
        }

        let mut parser = crate::parser::Parser::new(tokens, true);
        let program = parser.parse()?;

        if program.statements.is_empty() {
            return Ok(None);
        }

        // Si es una única expresión evaluable (ej: `10 + 20` o `x`), retornar el valor
        if program.statements.len() == 1 {
            let first = match &program.statements[0] {
                Statement::Located { stmt, .. } => stmt.as_ref(),
                other => other,
            };
            if let Statement::Expr(expr) = first {
                let val = self.eval_expression(expr)?;
                return Ok(Some(val));
            }
        }

        self.run(&program)?;
        Ok(None)
    }

    pub fn get_variables(&self) -> Vec<(String, DataType, Value)> {
        let mut vars = Vec::new();
        for (idx, scope) in self.scopes.iter().enumerate() {
            let mut keys: Vec<String> = scope.entries.keys().cloned().collect();
            keys.sort();
            for k in keys {
                if idx == 0 && k == "PI" {
                    continue;
                }
                let entry = scope.entries.get(&k).unwrap();
                vars.push((k, entry.var_type.clone(), entry.val.clone()));
            }
        }
        vars
    }

    pub fn reset(&mut self) {
        let mut initial_scope = Scope::new();
        initial_scope.entries.insert(
            "PI".to_string(),
            VariableEntry {
                val: Value::Real(std::f64::consts::PI),
                var_type: DataType::Real,
            },
        );
        self.scopes = vec![initial_scope];
        self.scope_pool.clear();
        self.functions.clear();
        self.struct_defs.clear();
        self.output.clear();
        self.call_stack.clear();
        self.current_line = 0;
        self.current_column = 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockKind {
    Inicio,
    Funcion,
    Procedimiento,
    Estructura,
    Si,
    Mientras,
    Hacer,
    Para,
}

pub fn is_input_complete(input: &str) -> bool {
    let mut lexer = crate::lexer::Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => return true,
    };

    let mut stack: Vec<BlockKind> = Vec::new();
    let mut parens: i32 = 0;
    let mut brackets: i32 = 0;
    let mut after_fin = false;
    let mut after_sino = false;

    for tok in tokens {
        match tok.token_type {
            crate::token::TokenType::ParentesisIzq => {
                parens += 1;
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::ParentesisDer => {
                if parens > 0 {
                    parens -= 1;
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::CorcheteIzq => {
                brackets += 1;
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::CorcheteDer => {
                if brackets > 0 {
                    brackets -= 1;
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwInicio => {
                stack.push(BlockKind::Inicio);
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwFuncion => {
                if !after_fin {
                    stack.push(BlockKind::Funcion);
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwProcedimiento => {
                if !after_fin {
                    stack.push(BlockKind::Procedimiento);
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwEstructura => {
                if !after_fin {
                    stack.push(BlockKind::Estructura);
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwSi => {
                if !after_fin && !after_sino {
                    stack.push(BlockKind::Si);
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwSino => {
                after_sino = true;
                after_fin = false;
            }
            crate::token::TokenType::KwMientras => {
                if !after_fin && stack.last() != Some(&BlockKind::Hacer) {
                    stack.push(BlockKind::Mientras);
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwHacer => {
                if !after_fin {
                    stack.push(BlockKind::Hacer);
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwPara => {
                if !after_fin {
                    stack.push(BlockKind::Para);
                }
                after_fin = false;
                after_sino = false;
            }
            crate::token::TokenType::KwFin => {
                let _ = stack.pop();
                after_fin = true;
                after_sino = false;
            }
            crate::token::TokenType::Eof => {}
            _ => {
                after_fin = false;
                after_sino = false;
            }
        }
    }

    stack.is_empty() && parens == 0 && brackets == 0
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}


fn num_to_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Entero(n) => Some(*n as f64),
        Value::Real(r) => Some(*r),
        Value::Flotante(f) => Some(*f as f64),
        _ => None,
    }
}

impl Interpreter {
    fn check_arity(&self, name: &str, args: &[Expression], n: usize) -> Result<(), YaguareteError> {
        if args.len() != n {
            let plural = if n == 1 { "argumento" } else { "argumentos" };
            return Err(self.runtime_error(format!("{} requiere exactamente {} {}", name, n, plural)));
        }
        Ok(())
    }

    fn text_arg(&self, name: &str, v: &Value) -> Result<String, YaguareteError> {
        match v {
            Value::Cadena(s) => Ok(s.clone()),
            Value::Caracter(c) => Ok(c.to_string()),
            _ => Err(self.runtime_error(format!("{} requiere un argumento de tipo CADENA", name))),
        }
    }

    fn parse_float_text(s: &str) -> Option<f64> {
        let t = s.trim();
        if t.is_empty() {
            return None;
        }
        let l = t.to_lowercase();
        if l.contains("inf") || l.contains("nan") {
            return None;
        }
        t.parse::<f64>().ok()
    }

    fn to_f64_conv(&self, v: &Value) -> Result<f64, YaguareteError> {
        let r = match v {
            Value::Entero(n) => Some(*n as f64),
            Value::Real(r) => Some(*r),
            Value::Flotante(f) => Some(*f as f64),
            Value::Booleano(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::Cadena(s) => Self::parse_float_text(s),
            Value::Caracter(c) => c.to_digit(10).map(|d| d as f64),
            _ => None,
        };
        r.ok_or_else(|| self.runtime_error(format!("No se puede convertir '{}' a número real", v.to_canonical_string())))
    }

    fn eval_text_builtin(&mut self, upper: &str, args: &[Expression]) -> Result<Option<Value>, YaguareteError> {
        match upper {
            "LEER" => {
                if args.len() > 1 {
                    return Err(self.runtime_error("LEER requiere 0 o 1 argumento"));
                }
                let prompt = if args.len() == 1 {
                    let v = self.eval_expression(&args[0])?;
                    match v {
                        Value::Cadena(_) | Value::Caracter(_) => self.text_arg("LEER", &v)?,
                        _ => return Err(self.runtime_error("LEER requiere un mensaje de tipo CADENA")),
                    }
                } else {
                    String::new()
                };
                if self.input_disabled {
                    return Err(self.runtime_error("LEER no está disponible durante la depuración (DAP)."));
                }
                let line = match self.input_reader.as_mut() {
                    Some(r) => r(&prompt),
                    None => {
                        use std::io::{BufRead, Write};
                        let mut out = std::io::stdout();
                        let _ = write!(out, "{}", prompt);
                        let _ = out.flush();
                        let mut buf = String::new();
                        match std::io::stdin().lock().read_line(&mut buf) {
                            Ok(0) | Err(_) => None,
                            Ok(_) => Some(buf),
                        }
                    }
                };
                let line = line.unwrap_or_default();
                let line = line.trim_end_matches(&['\r', '\n'][..]).to_string();
                Ok(Some(parse_input_line(&line)))
            }
            "MAYUSCULAS" | "MINUSCULAS" | "RECORTAR" => {
                self.check_arity(upper, args, 1)?;
                let v = self.eval_expression(&args[0])?;
                let s = self.text_arg(upper, &v)?;
                Ok(Some(Value::Cadena(match upper {
                    "MAYUSCULAS" => s.to_uppercase(),
                    "MINUSCULAS" => s.to_lowercase(),
                    _ => s.trim().to_string(),
                })))
            }
            "DIVIDIR_CADENA" => {
                self.check_arity(upper, args, 2)?;
                let a = self.eval_expression(&args[0])?;
                let b = self.eval_expression(&args[1])?;
                let s = self.text_arg(upper, &a)?;
                let sep = self.text_arg(upper, &b)?;
                if sep.is_empty() {
                    return Err(self.runtime_error("El separador no puede estar vacío."));
                }
                Ok(Some(Value::Vector(s.split(sep.as_str()).map(|p| Value::Cadena(p.to_string())).collect())))
            }
            "A_ENTERO" => {
                self.check_arity(upper, args, 1)?;
                let v = self.eval_expression(&args[0])?;
                let fail = |me: &Self| me.runtime_error(format!("No se puede convertir '{}' a ENTERO", v.to_canonical_string()));
                let n = match &v {
                    Value::Entero(n) => *n,
                    Value::Real(r) => r.trunc() as i64,
                    Value::Flotante(f) => f.trunc() as i64,
                    Value::Booleano(b) => *b as i64,
                    Value::Caracter(c) => match c.to_digit(10) {
                        Some(d) => d as i64,
                        None => return Err(fail(self)),
                    },
                    Value::Cadena(s) => {
                        let t = s.trim();
                        if let Ok(n) = t.parse::<i64>() {
                            n
                        } else if let Some(f) = Self::parse_float_text(t) {
                            f.trunc() as i64
                        } else {
                            return Err(fail(self));
                        }
                    }
                    _ => return Err(fail(self)),
                };
                Ok(Some(Value::Entero(n)))
            }
            "A_REAL" => {
                self.check_arity(upper, args, 1)?;
                let v = self.eval_expression(&args[0])?;
                Ok(Some(Value::Real(self.to_f64_conv(&v)?)))
            }
            "A_FLOTANTE" => {
                self.check_arity(upper, args, 1)?;
                let v = self.eval_expression(&args[0])?;
                Ok(Some(Value::Flotante(self.to_f64_conv(&v)? as f32)))
            }
            "A_CADENA" => {
                self.check_arity(upper, args, 1)?;
                let v = self.eval_expression(&args[0])?;
                Ok(Some(Value::Cadena(v.to_canonical_string())))
            }
            "A_BOOLEANO" => {
                self.check_arity(upper, args, 1)?;
                let v = self.eval_expression(&args[0])?;
                let b = match &v {
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
                    Some(b) => Ok(Some(Value::Booleano(b))),
                    None => Err(self.runtime_error(format!("No se puede convertir '{}' a BOOLEANO", v.to_canonical_string()))),
                }
            }
            "MIN" | "MAX" => {
                self.check_arity(upper, args, 2)?;
                let a = self.eval_expression(&args[0])?;
                let b = self.eval_expression(&args[1])?;
                let (x, y) = match (num_to_f64(&a), num_to_f64(&b)) {
                    (Some(x), Some(y)) => (x, y),
                    _ => return Err(self.runtime_error(format!("{} requiere argumentos numéricos", upper))),
                };
                let is_min = upper == "MIN";
                Ok(Some(match (&a, &b) {
                    (Value::Entero(p), Value::Entero(q)) => Value::Entero(if is_min { *p.min(q) } else { *p.max(q) }),
                    (Value::Flotante(p), Value::Flotante(q)) => Value::Flotante(if is_min { (*p).min(*q) } else { (*p).max(*q) }),
                    _ => Value::Real(if is_min { x.min(y) } else { x.max(y) }),
                }))
            }
            "REDONDEAR" => {
                if args.is_empty() || args.len() > 2 {
                    return Err(self.runtime_error("REDONDEAR requiere 1 o 2 argumentos"));
                }
                let v = self.eval_expression(&args[0])?;
                let x = num_to_f64(&v).ok_or_else(|| self.runtime_error("REDONDEAR requiere un argumento numérico"))?;
                if args.len() == 1 {
                    return Ok(Some(Value::Entero(x.round() as i64)));
                }
                let d = match self.eval_expression(&args[1])? {
                    Value::Entero(d) if d >= 0 => d,
                    _ => return Err(self.runtime_error("Los decimales de REDONDEAR deben ser un entero mayor o igual a 0")),
                };
                let f = 10f64.powi(d.min(300) as i32);
                let r = (x * f).round() / f;
                Ok(Some(Value::Real(if r.is_finite() { r } else { x })))
            }
            _ => Ok(None),
        }
    }
}
