pub mod chunk;
pub mod compiler;
pub mod opcode;
pub mod vm;

pub use chunk::Chunk;
pub use compiler::Compiler;
pub use opcode::OpCode;
pub use vm::VM;

use crate::error::YaguareteError;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn run_code_vm(source: &str, lenient: bool) -> Result<Vec<String>, YaguareteError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens, lenient);
    let program = parser.parse()?;

    let compiler = Compiler::new();
    let (chunk, functions, struct_defs) = compiler.compile_program(&program)?;

    let mut vm = VM::new();
    vm.run_program(chunk, functions, struct_defs)?;

    Ok(vm.output)
}
