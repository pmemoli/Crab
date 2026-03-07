#[derive(Debug)]
pub struct Program(Function);

#[derive(Debug)]
pub struct Function(String, Instruction);

#[derive(Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i32),
    Register,
}
