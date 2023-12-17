use crate::Id;
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    // Declare a new variable.
    Var,

    // A constant value.
    // Const(Vec<Vec<i64>>),

    // 我自己的操作符号
    TfAdd(Id, Id),
    TfCast(Id),
    TfConstant(Id),
    TfEqual(Id, Id),
    TfMultiply(Id, Id),
    TfSquare(Id),
    TfSubtract(Id, Id),
}

impl Operator {
    pub fn arity(&self) -> usize {
        match self {
            Operator::Var => 0,
            // | Operator::Const(_) => 0,
            Operator::TfCast(_)
            | Operator::TfConstant(_)
            | Operator::TfSquare(_)
            => 1,
            Operator::TfAdd(_, _)
            | Operator::TfEqual(_, _)
            | Operator::TfMultiply(_, _)
            | Operator::TfSubtract(_, _)
            => 2,
        }
    }

    // pub fn immediates(&self, mut f: impl FnMut(Vec<Vec<i64>>)) {
    //     if let Operator::Const(c) = self {
    //         f(c.to_vec());
    //     }
    // }

    pub fn operands(&self, mut f: impl FnMut(Id)) {
        match *self {
            Operator::Var 
            // | Operator::Const(_) 
            => {},
            Operator::TfCast(a)
            | Operator::TfConstant(a)
            | Operator::TfSquare(a)
            => {
                f(a);
            },
            Operator::TfAdd(a, b)
            | Operator::TfEqual(a, b)
            | Operator::TfMultiply(a, b)
            | Operator::TfSubtract(a, b)
            => {
                f(a);
                f(b);
            },
            // },
        }
    }

    pub fn operands_mut(&mut self, mut f: impl FnMut(&mut Id)) {
        match self {
            Operator::Var 
            // | Operator::Const(_) 
            => {},
            Operator::TfCast(a)
            | Operator::TfConstant(a)
            | Operator::TfSquare(a)
            => {
                f(a);
            },
            Operator::TfAdd(a, b)
            | Operator::TfEqual(a, b)
            | Operator::TfMultiply(a, b)
            | Operator::TfSubtract(a, b)
             => {
                f(a);
                f(b);
            },
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Var => write!(f, "var: vec"),
            //Operator::Const(c) => write!(f, "const: {:?}", c),
            Operator::TfAdd(a, b) => write!(f, "TfAdd: {}, {}", a, b),
            Operator::TfCast(a) => write!(f, "TfCast: {}", a),
            Operator::TfConstant(a) => write!(f, "TfConstant: {}", a),
            Operator::TfEqual(a, b) => write!(f, "TfEqual: {}, {}", a, b),
            Operator::TfMultiply(a, b) => write!(f, "TfMultiply: {}, {}", a, b),
            Operator::TfSquare(a) => write!(f, "TfSquare: {}", a),
            Operator::TfSubtract(a, b) => write!(f, "TfSubtract: {}, {}", a, b),
        }
    }
}
