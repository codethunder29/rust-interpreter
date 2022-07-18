#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Option<ExprLiteral>),
    Gropuing(Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>)
}

#[derive(Clone, Debug)]
pub enum ExprLiteral {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool)
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Minus,
    Bang
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash
}

pub struct Node {
    pub value: Expr,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>
}

impl Node {
    pub fn new(value: Expr) -> Self {
        Node {
            value,
            left_child: None,
            right_child: None
        }
    }

    pub fn get_left(&mut self) -> Option<&mut Box<Node>> {
        self.left_child.as_mut()
    }

    pub fn get_right(&mut self) -> Option<&mut Box<Node>> {
        self.right_child.as_mut()
    }

    pub fn set_left(&mut self, child: Box<Node>) {
        self.left_child = Some(child);
    }

    pub fn set_right(&mut self, child: Box<Node>) {
        self.right_child = Some(child);
    }
}