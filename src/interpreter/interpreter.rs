use crate::parser::{expr::*, Stmt};
use super::RuntimeError;

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null
}

impl Value {
    pub fn get_type(&self) -> String {
        match self {
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::String(_) => "String",
            Value::Bool(_) => "Bool",
            Value::Null => "Null"
        }.to_string()
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in statements {
            self.interpret_stmt(stmt)?;
        }

        Ok(())
    }
}

// helper functions
impl Interpreter {
    fn gen_error(&self, msg: &str) -> RuntimeError {
        RuntimeError {
            msg: msg.to_string(),
            token: None
        }
    }

    fn is_truthy(&self, literal: Value) -> bool {
        match literal {
            Value::Null => false,
            Value::Bool(val) => val,
            _ => true
        }
    }

    fn interpret_stmt(&self, stmt: Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Print(expr) => {
                let value = self.interpret_expr(expr)?;
                println!("{:?}", value);
            },
            Stmt::ExprStmt(expr) => {
                let value = self.interpret_expr(expr)?;
            },
        }

        Ok(())
    }

    fn interpret_expr(&self, expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(val) => self.interpret_literal(val),
            Expr::Gropuing(val) => self.interpret_expr(*val),
            Expr::Unary(op, val) => self.interpret_unary(op, *val),
            Expr::Binary(val1, op, val2) => self.interpret_binary(*val1, op, *val2),
        }
    }

    fn interpret_literal(&self, literal: Option<ExprLiteral>) -> Result<Value, RuntimeError> {
        if literal.is_none() {
            return Ok(Value::Null);
        }

        match literal.unwrap() {
            ExprLiteral::Int(val) => Ok(Value::Int(val)),
            ExprLiteral::Float(val) => Ok(Value::Float(val)),
            ExprLiteral::Str(val) => Ok(Value::String(val.clone())),
            ExprLiteral::Bool(val) => Ok(Value::Bool(val))
        }
    }

    fn interpret_unary(&self, op: UnaryOp, expr: Expr) -> Result<Value, RuntimeError> {
        let value = self.interpret_expr(expr)?;

        match op {
            UnaryOp::Minus => {
                match value {
                    Value::Int(val) => Ok(Value::Int(-val)),
                    Value::Float(val) => Ok(Value::Float(-val)),
                    Value::String(_) => Err(self.gen_error("Cannot apply '-' unary operator to String")),
                    Value::Bool(_) => Err(self.gen_error("Cannot apply '-' unary operator to Bool")),
                    Value::Null => Err(self.gen_error("Cannot apply '-' unary operator to Null")),
                }
            },
            UnaryOp::Bang => {
                match value {
                    Value::Int(_) => Err(self.gen_error("Cannot apply '!' unary operator to Int")),
                    Value::Float(_) => Err(self.gen_error("Cannot apply '!' unary operator to Float")),
                    Value::String(_) => Err(self.gen_error("Cannot apply '!' unary operator to String")),
                    Value::Bool(val) => Ok(Value::Bool(!val)),
                    Value::Null => Err(self.gen_error("Cannot apply '!' unary operator to Null")),
                }
            },
        }
    }

    fn interpret_binary(&self, left: Expr, op: BinaryOp, right: Expr) -> Result<Value, RuntimeError>{
        let left = self.interpret_expr(left)?;
        let right = self.interpret_expr(right)?;

        match op {
            BinaryOp::EqualEqual => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Bool(val1 == val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Bool(val1 == val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Bool(val1 as f64 == val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Bool(val1 == val2 as f64)),
                    (Value::String(val1), Value::String(val2)) => Ok(Value::Bool(val1 == val2)),
                    (Value::Bool(val1), Value::Bool(val2)) => Ok(Value::Bool(val1 == val2)),
                    (Value::Null, Value::Null) => Ok(Value::Bool(true)),
                    (Value::Null, _) => Ok(Value::Bool(false)),
                    (_, Value::Null) => Ok(Value::Bool(false)),
                    (val1, val2) => Err(self.gen_error(&format!("Cannot compare {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::BangEqual => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Bool(val1 != val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Bool(val1 != val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Bool(val1 as f64 != val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Bool(val1 != val2 as f64)),
                    (Value::String(val1), Value::String(val2)) => Ok(Value::Bool(val1 != val2)),
                    (Value::Bool(val1), Value::Bool(val2)) => Ok(Value::Bool(val1 != val2)),
                    (Value::Null, Value::Null) => Ok(Value::Bool(false)),
                    (Value::Null, _) => Ok(Value::Bool(true)),
                    (_, Value::Null) => Ok(Value::Bool(true)),
                    (val1, val2) => Err(self.gen_error(&format!("Cannot compare {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::Less => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Bool(val1 < val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Bool(val1 < val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Bool((val1 as f64) < val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Bool(val1 < val2 as f64)),
                    (Value::String(val1), Value::String(val2)) => Ok(Value::Bool(val1 < val2)),
                    (Value::Bool(val1), Value::Bool(val2)) => Ok(Value::Bool(val1 < val2)),
                    (val1, val2) => Err(self.gen_error(&format!("Cannot compare {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::LessEqual => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Bool(val1 <= val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Bool(val1 <= val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Bool((val1 as f64) <= val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Bool(val1 <= val2 as f64)),
                    (Value::String(val1), Value::String(val2)) => Ok(Value::Bool(val1 <= val2)),
                    (Value::Bool(val1), Value::Bool(val2)) => Ok(Value::Bool(val1 <= val2)),
                    (val1, val2) => Err(self.gen_error(&format!("Cannot compare {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::Greater => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Bool(val1 > val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Bool(val1 > val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Bool((val1 as f64) > val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Bool(val1 > val2 as f64)),
                    (Value::String(val1), Value::String(val2)) => Ok(Value::Bool(val1 > val2)),
                    (Value::Bool(val1), Value::Bool(val2)) => Ok(Value::Bool(val1 > val2)),
                    (val1, val2) => Err(self.gen_error(&format!("Cannot compare {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::GreaterEqual => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Bool(val1 >= val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Bool(val1 >= val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Bool((val1 as f64) >= val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Bool(val1 >= val2 as f64)),
                    (Value::String(val1), Value::String(val2)) => Ok(Value::Bool(val1 >= val2)),
                    (Value::Bool(val1), Value::Bool(val2)) => Ok(Value::Bool(val1 >= val2)),
                    (val1, val2) => Err(self.gen_error(&format!("Cannot compare {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::Plus => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Int(val1 + val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Float(val1 + val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Float(val1 as f64 + val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Float(val1 + val2 as f64)),
                    (Value::String(val1), Value::String(val2)) => Ok(Value::String(val1 + &val2)),
                    
                    (val1, val2) => Err(self.gen_error(&format!("Cannot add {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::Minus => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Int(val1 - val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Float(val1 - val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Float(val1 as f64 - val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Float(val1 - val2 as f64)),
                    
                    (val1, val2) => Err(self.gen_error(&format!("Cannot subtruct {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::Star => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => Ok(Value::Int(val1 * val2)),
                    (Value::Float(val1), Value::Float(val2)) => Ok(Value::Float(val1 * val2)),
                    (Value::Int(val1), Value::Float(val2)) => Ok(Value::Float(val1 as f64 * val2)),
                    (Value::Float(val1), Value::Int(val2)) => Ok(Value::Float(val1 * val2 as f64)),
                    (Value::String(val1), Value::Int(val2)) => {
                        let mut new_str = String::new();

                        for _i in 0..val2 {
                            new_str.push_str(&val1);
                        }

                        Ok(Value::String(new_str))
                    },
                    (Value::Int(val1), Value::String(val2)) => {
                        let mut new_str = String::new();

                        for _i in 0..val1 {
                            new_str.push_str(&val2);
                        }

                        Ok(Value::String(new_str))
                    },
                    
                    (val1, val2) => Err(self.gen_error(&format!("Cannot multiply {} and {}", val1.get_type(), val2.get_type())))
                }
            },
            BinaryOp::Slash => {
                match (left, right) {
                    (Value::Int(val1), Value::Int(val2)) => {
                        if val2 == 0 {
                            return Err(self.gen_error("Cannot divide by zero"));
                        }
                        
                        Ok(Value::Int(val1 / val2))
                    },
                    (Value::Float(val1), Value::Float(val2)) => {
                        if val2 == 0.0 {
                            return Err(self.gen_error("Cannot divide by zero"));
                        }
                        
                        Ok(Value::Float(val1 / val2))
                    },
                    (Value::Int(val1), Value::Float(val2)) => {
                        if val2 == 0.0 {
                            return Err(self.gen_error("Cannot divide by zero"));
                        }
                        
                        Ok(Value::Float(val1 as f64 / val2))
                    },
                    (Value::Float(val1), Value::Int(val2)) => {
                        if val2 == 0 {
                            return Err(self.gen_error("Cannot divide by zero"));
                        }
                        
                        Ok(Value::Float(val1 / val2 as f64))
                    },
                    
                    (val1, val2) => Err(self.gen_error(&format!("Cannot divide {} and {}", val1.get_type(), val2.get_type())))
                }
            }
        }
    }
}