use crate::Id;
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    // Declare a new variable.
    Var,

    // A constant value.
    Const([usize; 2]),

    // 我自己的操作符号

    // 绝对值，
    TfAbs(Id),
    // 相加
    TfAdd(Id, Id),
    // 相乘
    TfMul(Id, Id),
    // 相除
    TfDiv(Id, Id),
    // 数组最大值的下标
    TfArgMax(Id),
    // 数组最小值的下标
    TfArgMin(Id),
    // 掩码，即如果为1则返回原值，为0则什么也不做
    TfBooleanMask(Id, Id),
    // 类型转换，目前还是原样返回
    TfCast(Id),
    // 限制在最大值和最小值之间取值
    TfClipByValue(Id, Id, Id),
    // 根据给定的轴拼接数组
    TfConcat(Id, Id, Id),
    // 相等
    TfEqual(Id, Id),
    // 按照单位矩阵的位置填充1
    TfEye(Id, Id),
    // 填充0
    TfZeros(Id),
    // 填充1
    TfOnes(Id),
    // 把输入的内容都变成0
    TfZerosLike(Id),
    // 把输入的内容都变成1
    TfOnesLike(Id),
    // 填充
    TfFill(Id, Id),
    // 大于
    TfGreater(Id, Id),
    // 大于等于
    TfGreaterEqual(Id, Id),
    // 不等于
    TfNotEqual(Id, Id),
    // 相反数
    TfNegative(Id),
    // 倒数
    TfReciprocal(Id),
    // 记录数组里每个元素出现的次数
    TfBincount(Id, Id, Id, Id),
    // 统计非0个数出现的次数
    TfCountNonzero(Id),
    // 遍历依次累加求和，每次的结果放在每一项里
    TfCumsum(Id, Id, Id),
    // 两个数组每一项的最大值
    TfMaximum(Id, Id),
    // 两个数组每一项的最小值
    TfMinimum(Id, Id),
    // 顺序颠倒
    TfReverse(Id),
    // 确定符号
    TfSign(Id),
    // 每个数的平方
    TfSquare(Id),
    // 返回非0所在的位置，或者按照掩码返回第一个还是第二个数
    TfWhere(Id, Id, Id),
}

impl Operator {
    pub fn arity(&self) -> usize {
        match self {
            Operator::Var | Operator::Const(_) => 0,
            Operator::TfAbs(_) 
            | Operator::TfNegative(_) 
            | Operator::TfReciprocal(_)
            | Operator::TfCast(_) 
            | Operator::TfArgMax(_)
            | Operator::TfArgMin(_) 
            | Operator::TfCountNonzero(_) 
            | Operator::TfReverse(_) 
            | Operator::TfSign(_) 
            | Operator::TfSquare(_) 
            | Operator::TfOnes(_) 
            | Operator::TfZeros(_) 
            | Operator::TfOnesLike(_) 
            | Operator::TfZerosLike(_) => 1,
            Operator::TfAdd(_, _)
            | Operator::TfMul(_, _) 
            | Operator::TfDiv(_, _) 
            | Operator::TfBooleanMask(_, _) 
            | Operator::TfEqual(_, _)
            | Operator::TfEye(_, _)
            | Operator::TfFill(_, _) 
            | Operator::TfGreater(_, _) 
            | Operator::TfGreaterEqual(_, _) 
            | Operator::TfNotEqual(_, _) 
            | Operator::TfMaximum(_, _) 
            | Operator::TfMinimum(_, _) => 2,
            Operator::TfClipByValue(_, _, _) 
            | Operator::TfCumsum(_, _, _)
            | Operator::TfConcat(_, _, _) 
            | Operator::TfWhere(_, _, _) => 3,
            Operator::TfBincount(_, _, _, _) => 4,
        }
    }

    pub fn immediates(&self, mut f: impl FnMut([usize; 2])) {
        if let Operator::Const(c) = *self {
            f(c);
        }
    }

    pub fn operands(&self, mut f: impl FnMut(Id)) {
        match *self {
            Operator::Var | Operator::Const(_) => {},
            Operator::TfAbs(a) 
            | Operator::TfNegative(a) 
            | Operator::TfReciprocal(a)
            | Operator::TfCast(a)
            | Operator::TfArgMax(a)
            | Operator::TfArgMin(a) 
            | Operator::TfCountNonzero(a) 
            | Operator::TfReverse(a) 
            | Operator::TfSign(a) 
            | Operator::TfSquare(a) 
            | Operator::TfOnes(a) 
            | Operator::TfZeros(a) 
            | Operator::TfOnesLike(a) 
            | Operator::TfZerosLike(a) => f(a),
            Operator::TfAdd(a, b)
            | Operator::TfMul(a, b) 
            | Operator::TfDiv(a, b) 
            | Operator::TfBooleanMask(a, b) 
            | Operator::TfEqual(a, b) 
            | Operator::TfEye(a, b)
            | Operator::TfFill(a, b) 
            | Operator::TfGreater(a, b) 
            | Operator::TfGreaterEqual(a, b) 
            | Operator::TfNotEqual(a, b) 
            | Operator::TfMaximum(a, b) 
            | Operator::TfMinimum(a, b) => {
                f(a);
                f(b);
            },
            Operator::TfClipByValue(a, b, c) 
            | Operator::TfCumsum(a, b, c)
            | Operator::TfConcat(a, b, c) 
            | Operator::TfWhere(a, b, c) => {
                f(a);
                f(b);
                f(c);
            },
            Operator::TfBincount(a, b, c, d) => {
                f(a);
                f(b);
                f(c);
                f(d);
            },
        }
    }

    pub fn operands_mut(&mut self, mut f: impl FnMut(&mut Id)) {
        match self {
            Operator::Var | Operator::Const(_) => {}
            Operator::TfAbs(a) 
            | Operator::TfNegative(a) 
            | Operator::TfReciprocal(a)
            | Operator::TfCast(a)
            | Operator::TfArgMax(a)
            | Operator::TfArgMin(a) 
            | Operator::TfCountNonzero(a) 
            | Operator::TfReverse(a) 
            | Operator::TfSign(a) 
            | Operator::TfSquare(a) 
            | Operator::TfOnes(a) 
            | Operator::TfZeros(a) 
            | Operator::TfOnesLike(a) 
            | Operator::TfZerosLike(a) => f(a),
            Operator::TfAdd(a, b)
            | Operator::TfMul(a, b) 
            | Operator::TfDiv(a, b) 
            | Operator::TfBooleanMask(a, b) 
            | Operator::TfEqual(a, b) 
            | Operator::TfEye(a, b)
            | Operator::TfFill(a, b) 
            | Operator::TfGreater(a, b) 
            | Operator::TfGreaterEqual(a, b) 
            | Operator::TfNotEqual(a, b) 
            | Operator::TfMaximum(a, b) 
            | Operator::TfMinimum(a, b) => {
                f(a);
                f(b);
            },
            Operator::TfClipByValue(a, b, c) 
            | Operator::TfCumsum(a, b, c)
            | Operator::TfConcat(a, b, c) 
            | Operator::TfWhere(a, b, c) => {
                f(a);
                f(b);
                f(c);
            },
            Operator::TfBincount(a, b, c, d) => {
                f(a);
                f(b);
                f(c);
                f(d);
            },
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Var => write!(f, "var: vec"),
            Operator::Const(c) => write!(f, "const: dims_len {:#X}", c.len()),
            Operator::TfAbs(a) => write!(f, "TfAbs: {}", a),
            Operator::TfAdd(a, b) => write!(f, "TfAdd: {}, {}", a, b),
            Operator::TfMul(a, b) => write!(f, "TfMul: {}, {}", a, b),
            Operator::TfDiv(a, b) => write!(f, "TfDiv: {}, {}", a, b),
            Operator::TfArgMax(a) => write!(f, "TfArgMax: {}", a),
            Operator::TfArgMin(a) => write!(f, "TfArgMin: {}", a),
            Operator::TfBooleanMask(a, b) => write!(f, "TfBooleanMask: {}, {}", a, b),
            Operator::TfCast(a) => write!(f, "TfCast: {}", a),
            Operator::TfClipByValue(a, b, c) => write!(f, "TfClipByValue: {}, {}, {}", a, b, c),
            Operator::TfConcat(a, b, c) => write!(f, "TfConcat: {}, {}, {}", a, b, c),
            Operator::TfEqual(a, b) => write!(f, "TfEqual: {}, {}", a, b),
            Operator::TfEye(a, b) => write!(f, "TfEye: {}, {}", a, b),
            Operator::TfOnes(a) => write!(f, "TfOnes: {}", a),
            Operator::TfZeros(a) => write!(f, "TfZeros: {}", a),
            Operator::TfOnesLike(a) => write!(f, "TfOnesLike: {}", a),
            Operator::TfZerosLike(a) => write!(f, "TfZerosLike: {}", a),
            Operator::TfFill(a, b) => write!(f, "TfFill: {}, {}", a, b),
            Operator::TfGreater(a, b) => write!(f, "TfGreater: {}, {}", a, b),
            Operator::TfGreaterEqual(a, b) => write!(f, "TfGreaterEqual: {}, {}", a, b),
            Operator::TfNotEqual(a, b) => write!(f, "TfNotEqual: {}, {}", a, b),
            Operator::TfNegative(id) => write!(f, "TfNegative: {}", id),
            Operator::TfReciprocal(id) => write!(f, "TfReciprocal: {}", id),
            Operator::TfBincount(a, b, c, d) => write!(f, "TfBincount: {}, {}, {}, {}", a, b, c, d),
            Operator::TfCountNonzero(id) => write!(f, "TfCountNonzero: {}", id),
            Operator::TfCumsum(a, b, c) => write!(f, "TfCumsum: {}, {}, {}", a, b, c),
            Operator::TfMaximum(a, b) => write!(f, "TfMaximum, {}, {}", a, b),
            Operator::TfMinimum(a, b) => write!(f, "TfMinimum, {}, {}", a, b),
            Operator::TfReverse(a) => write!(f, "TfReverse: {}", a),
            Operator::TfSign(a) => write!(f, "TfSign: {}", a),
            Operator::TfSquare(a) => write!(f, "TfSquare: {}", a),
            Operator::TfWhere(a, b, c) => write!(f, "TfWhere: {}, {}, {}", a, b, c),
        }
    }
}
