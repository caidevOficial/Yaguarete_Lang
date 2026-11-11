#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Constantes y literales
    Constant,      // Operando: u16 (índice en pool de constantes)
    Nil,
    True,
    False,
    Int0,
    Int1,

    // Gestión de pila
    Pop,
    Dup,

    // Variables locales por slot O(1)
    GetLocal,      // Operando: u16 (slot de pila)
    SetLocal,      // Operando: u16 (slot de pila)
    IncLocal,      // Operando: u16 (slot de pila) - Fast path j++
    DecLocal,      // Operando: u16 (slot de pila) - Fast path j--

    // Variables globales
    GetGlobal,     // Operando: u16 (índice de nombre en constantes)
    SetGlobal,     // Operando: u16 (índice de nombre en constantes)
    DefineGlobal,  // Operando: u16 (índice de nombre en constantes)

    // Operadores aritméticos
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    Negate,

    // Comparaciones y lógica
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Not,

    // Saltos y bucles
    Jump,          // Operando: u16 (salto hacia adelante)
    JumpIfFalse,   // Operando: u16 (salto condicional hacia adelante)
    Loop,          // Operando: u16 (salto hacia atrás)

    // Funciones
    Call,          // Operando: u8 (cantidad de argumentos)
    Return,

    // Colecciones y estructuras
    BuildVector,   // Operando: u16 (cantidad de elementos en pila)
    GetIndex,
    GetIndexLocal, // Operando: u16 (slot de pila) - Fast path v[i] sin clonar vector
    GetIndexGlobal,// Operando: u16 (nombre en constantes) - Fast path v[i] sin clonar vector
    SetIndex,
    SetIndexLocal, // Operando: u16 (slot de pila) - Fast path v[i] = x
    SetIndexGlobal,// Operando: u16 (nombre en constantes)
    GetMember,     // Operando: u16 (índice del nombre de campo en constantes)
    SetMember,     // Operando: u16 (índice del nombre de campo en constantes)
    NewStruct,     // Operando: u16 (índice del nombre de estructura en constantes)
    Length,        // Fast path para .largo
    LengthLocal,   // Operando: u16 (slot de pila) - Fast path v.largo
    LengthGlobal,  // Operando: u16 (nombre en constantes) - Fast path v.largo

    // I/O y funciones incorporadas
    Print,         // Operando: u8 (cantidad de argumentos)
    Builtin,       // Operando: u8 (id de función nativa), Operando: u8 (argc)

    // Depuración y ubicación
    Location,      // Operando: u16 (línea), Operando: u16 (columna)
}

impl OpCode {
    #[inline(always)]
    pub fn from_u8(byte: u8) -> Option<Self> {
        if byte <= OpCode::Location as u8 {
            Some(unsafe { std::mem::transmute(byte) })
        } else {
            None
        }
    }
}
