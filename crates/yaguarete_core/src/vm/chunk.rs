use crate::value::Value;
use crate::vm::opcode::OpCode;

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
    pub columns: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn write_byte(&mut self, byte: u8, line: usize, col: usize) {
        self.code.push(byte);
        self.lines.push(line);
        self.columns.push(col);
    }

    #[inline(always)]
    pub fn write_opcode(&mut self, op: OpCode, line: usize, col: usize) {
        self.write_byte(op as u8, line, col);
    }

    #[inline(always)]
    pub fn write_u16(&mut self, val: u16, line: usize, col: usize) {
        self.write_byte((val >> 8) as u8, line, col);
        self.write_byte((val & 0xFF) as u8, line, col);
    }

    pub fn patch_u16(&mut self, offset: usize, val: u16) {
        self.code[offset] = (val >> 8) as u8;
        self.code[offset + 1] = (val & 0xFF) as u8;
    }

    #[inline(always)]
    pub fn read_u16(&self, offset: usize) -> u16 {
        ((self.code[offset] as u16) << 8) | (self.code[offset + 1] as u16)
    }

    pub fn add_constant(&mut self, value: Value) -> u16 {
        // Reutilizar constantes primitivas idénticas para optimizar el pool
        match &value {
            Value::Entero(_) | Value::Real(_) | Value::Flotante(_) | Value::Booleano(_) | Value::Cadena(_) => {
                for (i, c) in self.constants.iter().enumerate() {
                    if c == &value {
                        return i as u16;
                    }
                }
            }
            _ => {}
        }
        let idx = self.constants.len();
        self.constants.push(value);
        idx as u16
    }

    /// Desensamblador para inspección y depuración de bytecode
    pub fn disassemble(&self, name: &str) -> String {
        let mut out = format!("== Bytecode Chunk: {} ({} bytes, {} constantes) ==\n", name, self.code.len(), self.constants.len());
        let mut offset = 0;
        while offset < self.code.len() {
            out.push_str(&self.disassemble_instruction(&mut offset));
            out.push('\n');
        }
        out
    }

    pub fn disassemble_instruction(&self, offset: &mut usize) -> String {
        let curr = *offset;
        let line = self.lines.get(curr).copied().unwrap_or(0);
        let byte = self.code[curr];
        *offset += 1;

        let op = match OpCode::from_u8(byte) {
            Some(o) => o,
            None => return format!("{:04} | Desconocido (0x{:02x})", curr, byte),
        };

        match op {
            OpCode::Constant => {
                let idx = self.read_u16(*offset);
                *offset += 2;
                let val_str = self.constants.get(idx as usize).map(|v| v.to_canonical_string()).unwrap_or_else(|| "?".to_string());
                format!("{:04} [L{:03}] CONSTANT       #{:<4} ({})", curr, line, idx, val_str)
            }
            OpCode::Nil | OpCode::True | OpCode::False | OpCode::Int0 | OpCode::Int1 |
            OpCode::Pop | OpCode::Dup | OpCode::Add | OpCode::Sub | OpCode::Mul |
            OpCode::Div | OpCode::FloorDiv | OpCode::Mod | OpCode::Pow | OpCode::Negate |
            OpCode::Equal | OpCode::NotEqual | OpCode::Less | OpCode::Greater |
            OpCode::LessEqual | OpCode::GreaterEqual | OpCode::Not |
            OpCode::GetIndex | OpCode::SetIndex | OpCode::Length | OpCode::Return => {
                format!("{:04} [L{:03}] {:?}", curr, line, op)
            }
            OpCode::GetLocal | OpCode::SetLocal | OpCode::IncLocal | OpCode::DecLocal | OpCode::SetIndexLocal | OpCode::GetIndexLocal | OpCode::LengthLocal => {
                let slot = self.read_u16(*offset);
                *offset += 2;
                format!("{:04} [L{:03}] {:<14} slot={}", curr, line, format!("{:?}", op), slot)
            }
            OpCode::GetGlobal | OpCode::SetGlobal | OpCode::DefineGlobal | OpCode::GetMember | OpCode::SetMember | OpCode::NewStruct | OpCode::SetIndexGlobal | OpCode::GetIndexGlobal | OpCode::LengthGlobal => {
                let idx = self.read_u16(*offset);
                *offset += 2;
                let name = self.constants.get(idx as usize).map(|v| v.to_canonical_string()).unwrap_or_else(|| "?".to_string());
                format!("{:04} [L{:03}] {:<14} #{:<4} ({})", curr, line, format!("{:?}", op), idx, name)
            }
            OpCode::Jump | OpCode::JumpIfFalse => {
                let target = self.read_u16(*offset);
                *offset += 2;
                format!("{:04} [L{:03}] {:<14} -> {:04}", curr, line, format!("{:?}", op), target)
            }
            OpCode::Loop => {
                let target = self.read_u16(*offset);
                *offset += 2;
                format!("{:04} [L{:03}] LOOP           -> {:04}", curr, line, target)
            }
            OpCode::Call | OpCode::Print => {
                let argc = self.code[*offset];
                *offset += 1;
                format!("{:04} [L{:03}] {:<14} argc={}", curr, line, format!("{:?}", op), argc)
            }
            OpCode::Builtin => {
                let id = self.code[*offset];
                let argc = self.code[*offset + 1];
                *offset += 2;
                format!("{:04} [L{:03}] BUILTIN        id={}, argc={}", curr, line, id, argc)
            }
            OpCode::BuildVector => {
                let count = self.read_u16(*offset);
                *offset += 2;
                format!("{:04} [L{:03}] BUILD_VECTOR   count={}", curr, line, count)
            }
            OpCode::Location => {
                let l = self.read_u16(*offset);
                let c = self.read_u16(*offset + 2);
                *offset += 4;
                format!("{:04} [L{:03}] LOCATION       L{}:C{}", curr, line, l, c)
            }
        }
    }
}
