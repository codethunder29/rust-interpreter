use super::Expr;

#[derive(Clone, Debug)]
pub enum Stmt {
    Print(Expr),
    ExprStmt(Expr)
}