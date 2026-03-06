// Grammar defined with ASDL on grammar.md
struct Program(Function);

struct Function(String, Statement);

enum Statement {
    Return(Expr),
}

enum Expr {
    Constant(i32),
}

pub fn parse() {}
