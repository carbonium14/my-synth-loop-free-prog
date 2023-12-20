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
    TfArgmax(Id),
    TfCast(Id),
    TfConcat0(Id, Id),
    TfConcat1(Id, Id),
    TfConstant(Id),
    TfDivide(Id, Id),
    TfEqual(Id, Id),
    TfExpandDims(Id),
    TfGreater(Id, Id),
    TfBincount(Id),
    TfMultiply(Id, Id),
    TfRange(Id, Id),
    TfSequenceMask(Id),
    TfSquare(Id),
    TfSubtract(Id, Id),
    TfTensordot(Id, Id),
    TfTranspose(Id),

    TfEye(Id, Id),
    TfFill(Id, Id),
    TfMatmul(Id, Id),
    TfMaximum(Id, Id),
    TfMinimum(Id, Id),
    TfNotEqual(Id, Id),
    TfOnes(Id),
    TfRoll(Id),
    TfZeros(Id),
}

impl Operator {
    pub fn arity(&self) -> usize {
        match self {
            Operator::Var => 0,
            // | Operator::Const(_) => 0,

            Operator::TfArgmax(_)
            | Operator::TfCast(_)
            | Operator::TfConstant(_)
            | Operator::TfExpandDims(_)
            | Operator::TfBincount(_)
            | Operator::TfSequenceMask(_)
            | Operator::TfSquare(_)
            | Operator::TfTranspose(_)

            | Operator::TfOnes(_)
            | Operator::TfRoll(_)
            | Operator::TfZeros(_)
            => 1,
            Operator::TfAdd(_, _)
            | Operator::TfConcat0(_, _)
            | Operator::TfConcat1(_, _)
            | Operator::TfDivide(_, _)
            | Operator::TfEqual(_, _)
            | Operator::TfGreater(_, _)
            | Operator::TfMultiply(_, _)
            | Operator::TfRange(_, _)
            | Operator::TfSubtract(_, _)
            | Operator::TfTensordot(_, _)

            | Operator::TfEye(_, _)
            | Operator::TfFill(_, _)
            | Operator::TfMatmul(_, _)
            | Operator::TfMaximum(_, _)
            | Operator::TfMinimum(_, _)
            | Operator::TfNotEqual(_, _)
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
            Operator::TfArgmax(a)
            | Operator::TfCast(a)
            | Operator::TfConstant(a)
            | Operator::TfExpandDims(a)
            | Operator::TfBincount(a)
            | Operator::TfSequenceMask(a)
            | Operator::TfSquare(a)
            | Operator::TfTranspose(a)

            | Operator::TfOnes(a)
            | Operator::TfRoll(a)
            | Operator::TfZeros(a)
            => {
                f(a);
            },
            Operator::TfAdd(a, b)
            | Operator::TfConcat0(a, b)
            | Operator::TfConcat1(a, b)
            | Operator::TfDivide(a, b)
            | Operator::TfEqual(a, b)
            | Operator::TfGreater(a, b)
            | Operator::TfMultiply(a, b)
            | Operator::TfRange(a, b)
            | Operator::TfSubtract(a, b)
            | Operator::TfTensordot(a, b)

            | Operator::TfEye(a, b)
            | Operator::TfFill(a, b)
            | Operator::TfMatmul(a, b)
            | Operator::TfMaximum(a, b)
            | Operator::TfMinimum(a, b)
            | Operator::TfNotEqual(a, b)
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
            Operator::TfArgmax(a)
            | Operator::TfCast(a)
            | Operator::TfConstant(a)
            | Operator::TfExpandDims(a)
            | Operator::TfBincount(a)
            | Operator::TfSequenceMask(a)
            | Operator::TfSquare(a)
            | Operator::TfTranspose(a)

            | Operator::TfOnes(a)
            | Operator::TfRoll(a)
            | Operator::TfZeros(a)
            => {
                f(a);
            },
            Operator::TfAdd(a, b)
            | Operator::TfConcat0(a, b)
            | Operator::TfConcat1(a, b)
            | Operator::TfDivide(a, b)
            | Operator::TfEqual(a, b)
            | Operator::TfGreater(a, b)
            | Operator::TfMultiply(a, b)
            | Operator::TfRange(a, b)
            | Operator::TfSubtract(a, b)
            | Operator::TfTensordot(a, b)

            | Operator::TfEye(a, b)
            | Operator::TfFill(a, b)
            | Operator::TfMatmul(a, b)
            | Operator::TfMaximum(a, b)
            | Operator::TfMinimum(a, b)
            | Operator::TfNotEqual(a, b)
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
            Operator::TfArgmax(a) => write!(f, "TfArgmax: {}, axis = 1", a),
            Operator::TfCast(a) => write!(f, "TfCast: {}", a),
            Operator::TfConcat0(a, b) => write!(f, "TfConcat: {}, {}, axis = 0", a, b),
            Operator::TfConcat1(a, b) => write!(f, "TfConcat: {}, {}, axis = 1", a, b),
            Operator::TfConstant(a) => write!(f, "TfConstant: {}", a),
            Operator::TfDivide(a, b) => write!(f, "TfDivide: {}, {}", a, b),
            Operator::TfEqual(a, b) => write!(f, "TfEqual: {}, {}", a, b),
            Operator::TfExpandDims(a) => write!(f, "TfExpandDims: {}, axis = 1", a),
            Operator::TfGreater(a, b) => write!(f, "TfGreater: {}, {}", a, b),
            Operator::TfBincount(a) => write!(f, "TfBincount: {}", a),
            Operator::TfMultiply(a, b) => write!(f, "TfMultiply: {}, {}", a, b),
            Operator::TfRange(a, b) => write!(f, "TfRange: {}, {}", a, b),
            Operator::TfSequenceMask(a) => write!(f, "TfSequenceMask: {}", a),
            Operator::TfSquare(a) => write!(f, "TfSquare: {}", a),
            Operator::TfSubtract(a, b) => write!(f, "TfSubtract: {}, {}", a, b),
            Operator::TfTensordot(a, b) => write!(f, "TfTensordot: {}, {}", a, b),
            Operator::TfTranspose(a) => write!(f, "TfTranspose: {}", a),

            Operator::TfEye(a, b) => write!(f, "TfEye: {}, {}", a, b),
            Operator::TfFill(a, b) => write!(f, "TfFill: {}, {}", a, b),
            Operator::TfMatmul(a, b) => write!(f, "TfMatmul: {}, {}", a, b),
            Operator::TfMaximum(a, b) => write!(f, "TfMaximum: {}, {}", a, b),
            Operator::TfMinimum(a, b) => write!(f, "TfMinimum: {}, {}", a, b),
            Operator::TfNotEqual(a, b) => write!(f, "TfNotEqual: {}, {}", a, b),
            Operator::TfOnes(a) => write!(f, "TfOnes: {}", a),
            Operator::TfRoll(a) => write!(f, "TfRoll: {}", a),
            Operator::TfZeros(a) => write!(f, "TfZeros: {}", a),
        }
    }
}
