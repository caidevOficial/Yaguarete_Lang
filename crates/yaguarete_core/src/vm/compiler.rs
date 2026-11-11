use crate::ast::*;
use crate::error::YaguareteError;
use crate::value::Value;
use crate::vm::chunk::Chunk;
use crate::vm::opcode::OpCode;
use rustc_hash::FxHashMap;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BuiltinId {
    Abs = 1,
    RaizCuadrada = 2,
    Potencia = 3,
    Aleatorio = 4,
    EsPar = 5,
    EsImpar = 6,
    Mod = 7,
    Leer = 8,
    Mayusculas = 9,
    Minusculas = 10,
    Recortar = 11,
    DividirCadena = 12,
    AEntero = 13,
    AReal = 14,
    AFlotante = 15,
    ACadena = 16,
    ABooleano = 17,
    Min = 18,
    Max = 19,
    Redondear = 20,
    AbrirArchivo = 21,
}

#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
    pub return_type: DataType,
}

#[derive(Debug, Clone)]
struct Local {
    name: String,
    depth: usize,
    slot: u16,
}

pub struct Compiler {
    pub chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
    current_line: usize,
    current_col: usize,
    pub functions: Vec<Rc<CompiledFunction>>,
    pub struct_defs: FxHashMap<String, Vec<(String, DataType)>>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
            current_line: 1,
            current_col: 1,
            functions: Vec::new(),
            struct_defs: FxHashMap::default(),
        }
    }

    pub fn compile_program(mut self, program: &Program) -> Result<(Chunk, Vec<Rc<CompiledFunction>>, FxHashMap<String, Vec<(String, DataType)>>), YaguareteError> {
        // Pre-registrar constante PI en globales si no existe
        self.chunk.add_constant(Value::Real(std::f64::consts::PI));

        // 1. Registrar definiciones de estructuras
        for stmt in &program.statements {
            if let Statement::StructDef { name, fields } = stmt {
                self.struct_defs.insert(name.clone(), fields.clone());
            }
        }

        // 2. Compilar definiciones de funciones primero
        for stmt in &program.statements {
            if let Statement::FunctionDef { name, params, return_type, body } = stmt {
                let func = self.compile_function(name, params, return_type, body)?;
                let rc_func = Rc::new(func);
                self.functions.push(rc_func.clone());
            }
        }

        // 3. Compilar sentencias principales
        for stmt in &program.statements {
            if !matches!(stmt, Statement::FunctionDef { .. } | Statement::StructDef { .. }) {
                self.compile_statement(stmt)?;
            }
        }

        // Emitir fin de programa
        self.emit_opcode(OpCode::Return);
        Ok((self.chunk, self.functions, self.struct_defs))
    }

    fn compile_function(
        &mut self,
        name: &str,
        params: &[(String, DataType)],
        return_type: &DataType,
        body: &[Statement],
    ) -> Result<CompiledFunction, YaguareteError> {
        let mut fn_compiler = Compiler::new();
        fn_compiler.scope_depth = 1;
        fn_compiler.struct_defs = self.struct_defs.clone();

        // Declarar parámetros como locales en slots 0..params.len()
        for (p_name, _) in params {
            let slot = fn_compiler.locals.len() as u16;
            fn_compiler.locals.push(Local {
                name: p_name.clone(),
                depth: 1,
                slot,
            });
        }

        for s in body {
            fn_compiler.compile_statement(s)?;
        }

        // Si la función no finaliza con retorno explícito, retornar Nulo
        fn_compiler.emit_opcode(OpCode::Nil);
        fn_compiler.emit_opcode(OpCode::Return);

        Ok(CompiledFunction {
            name: name.to_string(),
            arity: params.len(),
            chunk: fn_compiler.chunk,
            return_type: return_type.clone(),
        })
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        while let Some(top) = self.locals.last() {
            if top.depth > self.scope_depth {
                self.emit_opcode(OpCode::Pop);
                self.locals.pop();
            } else {
                break;
            }
        }
    }

    fn resolve_local(&self, name: &str) -> Option<u16> {
        for local in self.locals.iter().rev() {
            if local.name == name {
                return Some(local.slot);
            }
        }
        None
    }

    #[inline(always)]
    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_byte(byte, self.current_line, self.current_col);
    }

    #[inline(always)]
    fn emit_opcode(&mut self, op: OpCode) {
        self.chunk.write_opcode(op, self.current_line, self.current_col);
    }

    #[inline(always)]
    fn emit_u16(&mut self, val: u16) {
        self.chunk.write_u16(val, self.current_line, self.current_col);
    }

    fn emit_constant(&mut self, value: Value) {
        match value {
            Value::Nulo => self.emit_opcode(OpCode::Nil),
            Value::Booleano(true) => self.emit_opcode(OpCode::True),
            Value::Booleano(false) => self.emit_opcode(OpCode::False),
            Value::Entero(0) => self.emit_opcode(OpCode::Int0),
            Value::Entero(1) => self.emit_opcode(OpCode::Int1),
            _ => {
                let idx = self.chunk.add_constant(value);
                self.emit_opcode(OpCode::Constant);
                self.emit_u16(idx);
            }
        }
    }

    fn emit_jump(&mut self, op: OpCode) -> usize {
        self.emit_opcode(op);
        let pos = self.chunk.code.len();
        self.emit_u16(0); // Placeholder
        pos
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len() as u16;
        self.chunk.patch_u16(offset, jump);
    }

    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_opcode(OpCode::Loop);
        let offset = loop_start as u16;
        self.emit_u16(offset);
    }

    fn compile_statement(&mut self, stmt: &Statement) -> Result<(), YaguareteError> {
        match stmt {
            Statement::Located { line, column, stmt } => {
                self.current_line = *line;
                self.current_col = *column;
                self.emit_opcode(OpCode::Location);
                self.emit_u16(*line as u16);
                self.emit_u16(*column as u16);
                self.compile_statement(stmt)
            }
            Statement::VarDecl { var_type, name, size_exprs, init } => {
                if name.eq_ignore_ascii_case("PI") {
                    return Err(YaguareteError::Runtime {
                        message: "No se puede reasignar la constante matemática 'PI'".into(),
                        line: self.current_line,
                        column: self.current_col,
                    });
                }

                if let Some(init_expr) = init {
                    self.compile_expression(init_expr)?;
                } else if !size_exprs.is_empty() {
                    // Creación de vector con tamaño definido
                    for s in size_exprs {
                        self.compile_expression(s)?;
                    }
                    if size_exprs.len() == 1 {
                        // Construye vector unidimensional con 0s por defecto
                        let _count_idx = self.chunk.add_constant(Value::Entero(size_exprs.len() as i64));
                        self.emit_opcode(OpCode::Builtin);
                        self.emit_byte(255); // Marcador para vector con tamaño dinámico
                        self.emit_byte(1);
                    } else {
                        // Matriz
                        self.emit_opcode(OpCode::Builtin);
                        self.emit_byte(254);
                        self.emit_byte(2);
                    }
                } else {
                    match var_type {
                        DataType::Estructura(struct_name) => {
                            let name_idx = self.chunk.add_constant(Value::Cadena(struct_name.clone()));
                            self.emit_opcode(OpCode::NewStruct);
                            self.emit_u16(name_idx);
                        }
                        DataType::Real => self.emit_constant(Value::Real(0.0)),
                        DataType::Flotante => self.emit_constant(Value::Flotante(0.0)),
                        DataType::Booleano => self.emit_opcode(OpCode::False),
                        DataType::Cadena => self.emit_constant(Value::Cadena(String::new())),
                        DataType::Caracter => self.emit_constant(Value::Caracter('\0')),
                        _ => self.emit_opcode(OpCode::Int0),
                    }
                }

                if self.scope_depth > 0 {
                    let slot = self.locals.len() as u16;
                    self.locals.push(Local {
                        name: name.clone(),
                        depth: self.scope_depth,
                        slot,
                    });
                    // El valor ya queda en la cima de la pila en la posición `slot`
                } else {
                    let name_idx = self.chunk.add_constant(Value::Cadena(name.clone()));
                    self.emit_opcode(OpCode::DefineGlobal);
                    self.emit_u16(name_idx);
                }
                Ok(())
            }
            Statement::Assign { target, val } => {
                match target {
                    Expression::Variable(name) => {
                        self.compile_expression(val)?;
                        if let Some(slot) = self.resolve_local(name) {
                            self.emit_opcode(OpCode::SetLocal);
                            self.emit_u16(slot);
                        } else {
                            let name_idx = self.chunk.add_constant(Value::Cadena(name.clone()));
                            self.emit_opcode(OpCode::SetGlobal);
                            self.emit_u16(name_idx);
                        }
                    }
                    Expression::IndexAccess { target: inner, index } => {
                        if let Expression::Variable(var_name) = inner.as_ref() {
                            if let Some(slot) = self.resolve_local(var_name) {
                                self.compile_expression(index)?;
                                self.compile_expression(val)?;
                                self.emit_opcode(OpCode::SetIndexLocal);
                                self.emit_u16(slot);
                            } else {
                                self.compile_expression(index)?;
                                self.compile_expression(val)?;
                                let name_idx = self.chunk.add_constant(Value::Cadena(var_name.clone()));
                                self.emit_opcode(OpCode::SetIndexGlobal);
                                self.emit_u16(name_idx);
                            }
                        } else {
                            self.compile_expression(inner)?;
                            self.compile_expression(index)?;
                            self.compile_expression(val)?;
                            self.emit_opcode(OpCode::SetIndex);
                        }
                    }
                    Expression::MemberAccess { target: inner, member } => {
                        self.compile_expression(inner)?;
                        self.compile_expression(val)?;
                        let mem_idx = self.chunk.add_constant(Value::Cadena(member.clone()));
                        self.emit_opcode(OpCode::SetMember);
                        self.emit_u16(mem_idx);
                    }
                    _ => {
                        return Err(YaguareteError::Runtime {
                            message: "Destino de asignación no válido en bytecode".into(),
                            line: self.current_line,
                            column: self.current_col,
                        });
                    }
                }
                Ok(())
            }
            Statement::AssignOp { target, op, val } => {
                match target {
                    Expression::Variable(name) => {
                        if let Some(slot) = self.resolve_local(name) {
                            self.emit_opcode(OpCode::GetLocal);
                            self.emit_u16(slot);
                            self.compile_expression(val)?;
                            self.compile_binary_op(*op);
                            self.emit_opcode(OpCode::SetLocal);
                            self.emit_u16(slot);
                        } else {
                            let name_idx = self.chunk.add_constant(Value::Cadena(name.clone()));
                            self.emit_opcode(OpCode::GetGlobal);
                            self.emit_u16(name_idx);
                            self.compile_expression(val)?;
                            self.compile_binary_op(*op);
                            self.emit_opcode(OpCode::SetGlobal);
                            self.emit_u16(name_idx);
                        }
                    }
                    _ => {
                        return Err(YaguareteError::Runtime {
                            message: "Operador de asignación compleja aún no soportado en bytecode".into(),
                            line: self.current_line,
                            column: self.current_col,
                        });
                    }
                }
                Ok(())
            }
            Statement::Increment { target, is_inc } => {
                if let Expression::Variable(name) = target {
                    if let Some(slot) = self.resolve_local(name) {
                        if *is_inc {
                            self.emit_opcode(OpCode::IncLocal);
                        } else {
                            self.emit_opcode(OpCode::DecLocal);
                        }
                        self.emit_u16(slot);
                        return Ok(());
                    } else {
                        let name_idx = self.chunk.add_constant(Value::Cadena(name.clone()));
                        self.emit_opcode(OpCode::GetGlobal);
                        self.emit_u16(name_idx);
                        self.emit_opcode(OpCode::Int1);
                        if *is_inc {
                            self.emit_opcode(OpCode::Add);
                        } else {
                            self.emit_opcode(OpCode::Sub);
                        }
                        self.emit_opcode(OpCode::SetGlobal);
                        self.emit_u16(name_idx);
                        return Ok(());
                    }
                }
                Err(YaguareteError::Runtime {
                    message: "Incremento solo soportado sobre variables en bytecode".into(),
                    line: self.current_line,
                    column: self.current_col,
                })
            }
            Statement::If { condition, then_branch, else_if_branches, else_branch } => {
                self.compile_expression(condition)?;
                let then_jump = self.emit_jump(OpCode::JumpIfFalse);
                self.emit_opcode(OpCode::Pop); // Quitar condición

                self.begin_scope();
                for s in then_branch {
                    self.compile_statement(s)?;
                }
                self.end_scope();

                let mut exit_jumps = Vec::new();
                exit_jumps.push(self.emit_jump(OpCode::Jump));

                self.patch_jump(then_jump);
                self.emit_opcode(OpCode::Pop); // Quitar condición

                for (e_cond, e_body) in else_if_branches {
                    self.compile_expression(e_cond)?;
                    let e_jump = self.emit_jump(OpCode::JumpIfFalse);
                    self.emit_opcode(OpCode::Pop);

                    self.begin_scope();
                    for s in e_body {
                        self.compile_statement(s)?;
                    }
                    self.end_scope();

                    exit_jumps.push(self.emit_jump(OpCode::Jump));
                    self.patch_jump(e_jump);
                    self.emit_opcode(OpCode::Pop);
                }

                if let Some(else_stmts) = else_branch {
                    self.begin_scope();
                    for s in else_stmts {
                        self.compile_statement(s)?;
                    }
                    self.end_scope();
                }

                for j in exit_jumps {
                    self.patch_jump(j);
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                self.compile_expression(condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
                self.emit_opcode(OpCode::Pop);

                self.begin_scope();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.end_scope();

                self.emit_loop(loop_start);
                self.patch_jump(exit_jump);
                self.emit_opcode(OpCode::Pop);
                Ok(())
            }
            Statement::DoWhile { body, condition } => {
                let loop_start = self.chunk.code.len();
                self.begin_scope();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.end_scope();

                self.compile_expression(condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
                self.emit_opcode(OpCode::Pop);
                self.emit_loop(loop_start);
                self.patch_jump(exit_jump);
                self.emit_opcode(OpCode::Pop);
                Ok(())
            }
            Statement::For { init, condition, update, body } => {
                self.begin_scope();
                self.compile_statement(init)?;

                let loop_start = self.chunk.code.len();
                self.compile_expression(condition)?;
                let exit_jump = self.emit_jump(OpCode::JumpIfFalse);
                self.emit_opcode(OpCode::Pop);

                self.begin_scope();
                for s in body {
                    self.compile_statement(s)?;
                }
                self.end_scope();

                self.compile_statement(update)?;

                self.emit_loop(loop_start);
                self.patch_jump(exit_jump);
                self.emit_opcode(OpCode::Pop);

                self.end_scope();
                Ok(())
            }
            Statement::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.compile_expression(expr)?;
                } else {
                    self.emit_opcode(OpCode::Nil);
                }
                self.emit_opcode(OpCode::Return);
                Ok(())
            }
            Statement::Print(args) => {
                for arg in args {
                    self.compile_expression(arg)?;
                }
                self.emit_opcode(OpCode::Print);
                self.emit_byte(args.len() as u8);
                Ok(())
            }
            Statement::Expr(expr) => {
                self.compile_expression(expr)?;
                self.emit_opcode(OpCode::Pop);
                Ok(())
            }
            Statement::FunctionDef { .. } | Statement::StructDef { .. } => Ok(()),
        }
    }

    fn compile_expression(&mut self, expr: &Expression) -> Result<(), YaguareteError> {
        match expr {
            Expression::LiteralInt(n) => self.emit_constant(Value::Entero(*n)),
            Expression::LiteralReal(r) => self.emit_constant(Value::Real(*r)),
            Expression::LiteralCadena(s) => self.emit_constant(Value::Cadena(s.clone())),
            Expression::LiteralCaracter(c) => self.emit_constant(Value::Caracter(*c)),
            Expression::LiteralBool(b) => self.emit_constant(Value::Booleano(*b)),
            Expression::LiteralNulo => self.emit_opcode(OpCode::Nil),
            Expression::LiteralVector(elements) => {
                for elem in elements {
                    self.compile_expression(elem)?;
                }
                self.emit_opcode(OpCode::BuildVector);
                self.emit_u16(elements.len() as u16);
            }
            Expression::Variable(name) => {
                if name.eq_ignore_ascii_case("PI") {
                    self.emit_constant(Value::Real(std::f64::consts::PI));
                    return Ok(());
                }
                if let Some(slot) = self.resolve_local(name) {
                    self.emit_opcode(OpCode::GetLocal);
                    self.emit_u16(slot);
                } else {
                    let name_idx = self.chunk.add_constant(Value::Cadena(name.clone()));
                    self.emit_opcode(OpCode::GetGlobal);
                    self.emit_u16(name_idx);
                }
            }
            Expression::Binary { left, op, right } => {
                match op {
                    BinaryOp::Y => {
                        self.compile_expression(left)?;
                        let end_jump = self.emit_jump(OpCode::JumpIfFalse);
                        self.emit_opcode(OpCode::Pop);
                        self.compile_expression(right)?;
                        self.patch_jump(end_jump);
                    }
                    BinaryOp::O => {
                        self.compile_expression(left)?;
                        let else_jump = self.emit_jump(OpCode::JumpIfFalse);
                        let end_jump = self.emit_jump(OpCode::Jump);
                        self.patch_jump(else_jump);
                        self.emit_opcode(OpCode::Pop);
                        self.compile_expression(right)?;
                        self.patch_jump(end_jump);
                    }
                    _ => {
                        self.compile_expression(left)?;
                        self.compile_expression(right)?;
                        self.compile_binary_op(*op);
                    }
                }
            }
            Expression::Unary { op, expr } => {
                self.compile_expression(expr)?;
                match op {
                    UnaryOp::No => self.emit_opcode(OpCode::Not),
                    UnaryOp::Negativo => self.emit_opcode(OpCode::Negate),
                    UnaryOp::Positivo => {} // No-op
                }
            }
            Expression::IndexAccess { target, index } => {
                if let Expression::Variable(name) = target.as_ref() {
                    if let Some(slot) = self.resolve_local(name) {
                        self.compile_expression(index)?;
                        self.emit_opcode(OpCode::GetIndexLocal);
                        self.emit_u16(slot);
                    } else {
                        self.compile_expression(index)?;
                        let name_idx = self.chunk.add_constant(Value::Cadena(name.clone()));
                        self.emit_opcode(OpCode::GetIndexGlobal);
                        self.emit_u16(name_idx);
                    }
                } else {
                    self.compile_expression(target)?;
                    self.compile_expression(index)?;
                    self.emit_opcode(OpCode::GetIndex);
                }
            }
            Expression::MemberAccess { target, member } => {
                if member == "largo" {
                    if let Expression::Variable(name) = target.as_ref() {
                        if let Some(slot) = self.resolve_local(name) {
                            self.emit_opcode(OpCode::LengthLocal);
                            self.emit_u16(slot);
                        } else {
                            let name_idx = self.chunk.add_constant(Value::Cadena(name.clone()));
                            self.emit_opcode(OpCode::LengthGlobal);
                            self.emit_u16(name_idx);
                        }
                    } else {
                        self.compile_expression(target)?;
                        self.emit_opcode(OpCode::Length);
                    }
                } else {
                    self.compile_expression(target)?;
                    let mem_idx = self.chunk.add_constant(Value::Cadena(member.clone()));
                    self.emit_opcode(OpCode::GetMember);
                    self.emit_u16(mem_idx);
                }
            }
            Expression::Call { name, args } => {
                let upper = name.to_uppercase();
                if let Some(builtin_id) = Self::match_builtin(&upper) {
                    for arg in args {
                        self.compile_expression(arg)?;
                    }
                    self.emit_opcode(OpCode::Builtin);
                    self.emit_byte(builtin_id as u8);
                    self.emit_byte(args.len() as u8);
                    return Ok(());
                }

                // Llamada a función de usuario
                let fn_name_idx = self.chunk.add_constant(Value::Cadena(upper));
                self.emit_opcode(OpCode::GetGlobal);
                self.emit_u16(fn_name_idx);
                for arg in args {
                    self.compile_expression(arg)?;
                }
                self.emit_opcode(OpCode::Call);
                self.emit_byte(args.len() as u8);
            }
            Expression::MethodCall { target: _, method: _, args: _ } => {
                return Err(YaguareteError::Runtime {
                    message: "Llamada a método aún no implementada en bytecode".into(),
                    line: self.current_line,
                    column: self.current_col,
                });
            }
        }
        Ok(())
    }

    fn compile_binary_op(&mut self, op: BinaryOp) {
        match op {
            BinaryOp::Suma => self.emit_opcode(OpCode::Add),
            BinaryOp::Resta => self.emit_opcode(OpCode::Sub),
            BinaryOp::Mult => self.emit_opcode(OpCode::Mul),
            BinaryOp::Div => self.emit_opcode(OpCode::Div),
            BinaryOp::DivEntera => self.emit_opcode(OpCode::FloorDiv),
            BinaryOp::Mod => self.emit_opcode(OpCode::Mod),
            BinaryOp::Potencia => self.emit_opcode(OpCode::Pow),
            BinaryOp::IgualQue => self.emit_opcode(OpCode::Equal),
            BinaryOp::Distinto => self.emit_opcode(OpCode::NotEqual),
            BinaryOp::Menor => self.emit_opcode(OpCode::Less),
            BinaryOp::Mayor => self.emit_opcode(OpCode::Greater),
            BinaryOp::MenorIgual => self.emit_opcode(OpCode::LessEqual),
            BinaryOp::MayorIgual => self.emit_opcode(OpCode::GreaterEqual),
            BinaryOp::Y | BinaryOp::O => {} // Manejados con cortocircuito
        }
    }

    fn match_builtin(name: &str) -> Option<BuiltinId> {
        match name {
            "ABS" => Some(BuiltinId::Abs),
            "RAIZCUADRADA" => Some(BuiltinId::RaizCuadrada),
            "POTENCIA" => Some(BuiltinId::Potencia),
            "ALEATORIO" => Some(BuiltinId::Aleatorio),
            "ESPAR" => Some(BuiltinId::EsPar),
            "ESIMPAR" => Some(BuiltinId::EsImpar),
            "MOD" => Some(BuiltinId::Mod),
            "LEER" => Some(BuiltinId::Leer),
            "MAYUSCULAS" => Some(BuiltinId::Mayusculas),
            "MINUSCULAS" => Some(BuiltinId::Minusculas),
            "RECORTAR" => Some(BuiltinId::Recortar),
            "DIVIDIR_CADENA" => Some(BuiltinId::DividirCadena),
            "A_ENTERO" => Some(BuiltinId::AEntero),
            "A_REAL" => Some(BuiltinId::AReal),
            "A_FLOTANTE" => Some(BuiltinId::AFlotante),
            "A_CADENA" => Some(BuiltinId::ACadena),
            "A_BOOLEANO" => Some(BuiltinId::ABooleano),
            "MIN" => Some(BuiltinId::Min),
            "MAX" => Some(BuiltinId::Max),
            "REDONDEAR" => Some(BuiltinId::Redondear),
            "ABRIR_ARCHIVO" => Some(BuiltinId::AbrirArchivo),
            _ => None,
        }
    }
}
