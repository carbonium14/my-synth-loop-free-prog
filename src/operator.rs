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
    // 第一个是两个输入维度相同的情况，第二个是不同的情况
    TfBooleanMask(Id, Id),
    TfBooleanMask_(Id, Id),
    TfCast(Id),
    // 下标表示不同的axis
    TfConcat0(Id, Id),
    TfConcat1(Id, Id),
    TfConstant(Id),
    TfDivide(Id, Id),
    TfEqual(Id, Id),
    TfExpandDims(Id),
    TfGreater(Id, Id),
    TfBincount(Id),
    TfCumsum(Id, Id),
    TfMultiply(Id, Id),
    TfOneHot(Id, Id),
    TfRange(Id, Id),
    // 下标表示不同的axis，没有下标就是axis=-1
    TfReduceMax(Id),
    TfReduceMax0(Id),
    TfReduceMax1(Id),
    TfReduceSum(Id),
    TfReduceSum0(Id),
    TfReduceSum1(Id),
    TfSequenceMask(Id),
    TfSquare(Id),
    TfSubtract(Id, Id),
    TfTensordot(Id, Id),
    TfTranspose(Id),
    // 下标表示参数的个数
    TfWhere1(Id),
    TfWhere3(Id, Id, Id),

    TfEye(Id, Id),
    TfFill(Id, Id),
    TfSegmentMax(Id, Id),
    TfMatmul(Id, Id),
    TfMaximum(Id, Id),
    TfMinimum(Id, Id),
    TfNotEqual(Id, Id),
    TfOnes(Id),
    // 下标表示不同的axis，没有下标就是axis=-1
    TfReduceAny0(Id),
    TfReduceAny1(Id),
    TfReduceMean(Id),
    TfReduceProd(Id),
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
            | Operator::TfReduceMax(_)
            | Operator::TfReduceMax0(_)
            | Operator::TfReduceMax1(_)
            | Operator::TfReduceSum(_)
            | Operator::TfReduceSum0(_)
            | Operator::TfReduceSum1(_)
            | Operator::TfSequenceMask(_)
            | Operator::TfSquare(_)
            | Operator::TfTranspose(_)
            | Operator::TfWhere1(_)

            | Operator::TfOnes(_)
            | Operator::TfReduceAny0(_)
            | Operator::TfReduceAny1(_)
            | Operator::TfReduceMean(_)
            | Operator::TfReduceProd(_)
            | Operator::TfRoll(_)
            | Operator::TfZeros(_)
            => 1,
            Operator::TfAdd(_, _)
            | Operator::TfBooleanMask(_, _)
            | Operator::TfBooleanMask_(_, _)
            | Operator::TfConcat0(_, _)
            | Operator::TfConcat1(_, _)
            | Operator::TfDivide(_, _)
            | Operator::TfEqual(_, _)
            | Operator::TfGreater(_, _)
            | Operator::TfCumsum(_, _)
            | Operator::TfMultiply(_, _)
            | Operator::TfOneHot(_, _)
            | Operator::TfRange(_, _)
            | Operator::TfSubtract(_, _)
            | Operator::TfTensordot(_, _)

            | Operator::TfEye(_, _)
            | Operator::TfFill(_, _)
            | Operator::TfSegmentMax(_, _)
            | Operator::TfMatmul(_, _)
            | Operator::TfMaximum(_, _)
            | Operator::TfMinimum(_, _)
            | Operator::TfNotEqual(_, _)
            => 2,
            | Operator::TfWhere3(_, _, _)
            => 3,
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
            | Operator::TfReduceMax(a)
            | Operator::TfReduceMax0(a)
            | Operator::TfReduceMax1(a)
            | Operator::TfReduceSum(a)
            | Operator::TfReduceSum0(a)
            | Operator::TfReduceSum1(a)
            | Operator::TfSequenceMask(a)
            | Operator::TfSquare(a)
            | Operator::TfTranspose(a)
            | Operator::TfWhere1(a)

            | Operator::TfOnes(a)
            | Operator::TfReduceAny0(a)
            | Operator::TfReduceAny1(a)
            | Operator::TfReduceMean(a)
            | Operator::TfReduceProd(a)
            | Operator::TfRoll(a)
            | Operator::TfZeros(a)
            => {
                f(a);
            },
            Operator::TfAdd(a, b)
            | Operator::TfBooleanMask(a, b)
            | Operator::TfBooleanMask_(a, b)
            | Operator::TfConcat0(a, b)
            | Operator::TfConcat1(a, b)
            | Operator::TfDivide(a, b)
            | Operator::TfEqual(a, b)
            | Operator::TfGreater(a, b)
            | Operator::TfCumsum(a, b)
            | Operator::TfMultiply(a, b)
            | Operator::TfOneHot(a, b)
            | Operator::TfRange(a, b)
            | Operator::TfSubtract(a, b)
            | Operator::TfTensordot(a, b)

            | Operator::TfEye(a, b)
            | Operator::TfFill(a, b)
            | Operator::TfSegmentMax(a, b)
            | Operator::TfMatmul(a, b)
            | Operator::TfMaximum(a, b)
            | Operator::TfMinimum(a, b)
            | Operator::TfNotEqual(a, b)
            => {
                f(a);
                f(b);
            },
            | Operator::TfWhere3(a, b, c)
            => {
                f(a);
                f(b);
                f(c);
            },
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
            | Operator::TfReduceMax(a)
            | Operator::TfReduceMax0(a)
            | Operator::TfReduceMax1(a)
            | Operator::TfReduceSum(a)
            | Operator::TfReduceSum0(a)
            | Operator::TfReduceSum1(a)
            | Operator::TfSequenceMask(a)
            | Operator::TfSquare(a)
            | Operator::TfTranspose(a)
            | Operator::TfWhere1(a)

            | Operator::TfOnes(a)
            | Operator::TfReduceAny0(a)
            | Operator::TfReduceAny1(a)
            | Operator::TfReduceMean(a)
            | Operator::TfReduceProd(a)
            | Operator::TfRoll(a)
            | Operator::TfZeros(a)
            => {
                f(a);
            },
            Operator::TfAdd(a, b)
            | Operator::TfBooleanMask(a, b)
            | Operator::TfBooleanMask_(a, b)
            | Operator::TfConcat0(a, b)
            | Operator::TfConcat1(a, b)
            | Operator::TfDivide(a, b)
            | Operator::TfEqual(a, b)
            | Operator::TfGreater(a, b)
            | Operator::TfCumsum(a, b)
            | Operator::TfMultiply(a, b)
            | Operator::TfOneHot(a, b)
            | Operator::TfRange(a, b)
            | Operator::TfSubtract(a, b)
            | Operator::TfTensordot(a, b)

            | Operator::TfEye(a, b)
            | Operator::TfFill(a, b)
            | Operator::TfSegmentMax(a, b)
            | Operator::TfMatmul(a, b)
            | Operator::TfMaximum(a, b)
            | Operator::TfMinimum(a, b)
            | Operator::TfNotEqual(a, b)
             => {
                f(a);
                f(b);
            },
            | Operator::TfWhere3(a, b, c)
            => {
                f(a);
                f(b);
                f(c);
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
            Operator::TfBooleanMask(a, b) => write!(f, "TfBooleanMask: {}, {}", a, b),
            Operator::TfBooleanMask_(a, b) => write!(f, "TfBooleanMask: {}, {}", a, b),
            Operator::TfCast(a) => write!(f, "TfCast: {}", a),
            Operator::TfConcat0(a, b) => write!(f, "TfConcat: {}, {}, axis = 0", a, b),
            Operator::TfConcat1(a, b) => write!(f, "TfConcat: {}, {}, axis = 1", a, b),
            Operator::TfConstant(a) => write!(f, "TfConstant: {}", a),
            Operator::TfDivide(a, b) => write!(f, "TfDivide: {}, {}", a, b),
            Operator::TfEqual(a, b) => write!(f, "TfEqual: {}, {}", a, b),
            Operator::TfExpandDims(a) => write!(f, "TfExpandDims: {}, axis = 1", a),
            Operator::TfGreater(a, b) => write!(f, "TfGreater: {}, {}", a, b),
            Operator::TfBincount(a) => write!(f, "TfBincount: {}", a),
            Operator::TfCumsum(a, b) => write!(f, "TfCumsum: {}, {}", a, b),
            Operator::TfMultiply(a, b) => write!(f, "TfMultiply: {}, {}", a, b),
            Operator::TfOneHot(a, b) => write!(f, "TfOneHot: {}, {}", a, b),
            Operator::TfRange(a, b) => write!(f, "TfRange: {}, {}", a, b),
            Operator::TfReduceMax(a) => write!(f, "TfReduceMax: {}", a),
            Operator::TfReduceMax0(a) => write!(f, "TfReduceMax: {}, axis = 0", a),
            Operator::TfReduceMax1(a) => write!(f, "TfReduceMax: {}, axis = 1", a),
            Operator::TfReduceSum(a) => write!(f, "TfReduceSum: {}", a),
            Operator::TfReduceSum0(a) => write!(f, "TfReduceSum: {}, axis = 0", a),
            Operator::TfReduceSum1(a) => write!(f, "TfReduceSum: {}, axis = 1", a),
            Operator::TfSequenceMask(a) => write!(f, "TfSequenceMask: {}", a),
            Operator::TfSquare(a) => write!(f, "TfSquare: {}", a),
            Operator::TfSubtract(a, b) => write!(f, "TfSubtract: {}, {}", a, b),
            Operator::TfTensordot(a, b) => write!(f, "TfTensordot: {}, {}", a, b),
            Operator::TfTranspose(a) => write!(f, "TfTranspose: {}", a),
            Operator::TfWhere1(a) => write!(f, "TfWhere: {}", a),
            Operator::TfWhere3(a, b, c) => write!(f, "TfWhere: {}, {}, {}", a, b, c),

            Operator::TfEye(a, b) => write!(f, "TfEye: {}, {}", a, b),
            Operator::TfFill(a, b) => write!(f, "TfFill: {}, {}", a, b),
            Operator::TfSegmentMax(a, b) => write!(f, "TfSegmentMax: {}, {}", a, b),
            Operator::TfMatmul(a, b) => write!(f, "TfMatmul: {}, {}", a, b),
            Operator::TfMaximum(a, b) => write!(f, "TfMaximum: {}, {}", a, b),
            Operator::TfMinimum(a, b) => write!(f, "TfMinimum: {}, {}", a, b),
            Operator::TfNotEqual(a, b) => write!(f, "TfNotEqual: {}, {}", a, b),
            Operator::TfOnes(a) => write!(f, "TfOnes: {}", a),
            Operator::TfReduceAny0(a) => write!(f, "TfReduceAny: {}, axis = 0", a),
            Operator::TfReduceAny1(a) => write!(f, "TfReduceAny: {}, axis = 1", a),
            Operator::TfReduceMean(a) => write!(f, "TfReduceMean: {}, axis = 0", a),
            Operator::TfReduceProd(a) => write!(f, "TfReduceProd: {}, axis = 1", a),
            Operator::TfRoll(a) => write!(f, "TfRoll: {}", a),
            Operator::TfZeros(a) => write!(f, "TfZeros: {}", a),
        }
    }
}
