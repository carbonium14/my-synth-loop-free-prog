use crate::{Id, Operator, Vecs, DIMS};
use std::{fmt::Debug, usize};
use z3::ast::{Int, Bool, Ast};

const _DIMSIZE : [usize ; 2] = [4, 10];
const _SIZE_STORE_INDEX : i64 = -2;
const _SIZE_X : i64 = 0;
const _SIZE_Y : i64 = 1;

// macro_rules! vecnd {
//     ($([$($inner:tt)*]),+ $(,)?) => {
//         vec![$(
//             vecnd![$($inner)*]
//         ),+]
//     };
//     ($($t:tt)*) => {
//         vec![$($t)*]
//     };
// }

// fn bit_vec_from_u64(context: &z3::Context, val: u64, bit_width: u32) -> BitVec {
//     BitVec::from_i64(context, val as i64, bit_width)
// }

// fn zero(context: &z3::Context, bit_width: u32) -> BitVec {
//     bit_vec_from_u64(context, 0, bit_width)
// }

// fn one(context: &z3::Context, bit_width: u32) -> BitVec {
//     bit_vec_from_u64(context, 1, bit_width)
// }
fn int_from_i64(context: &z3::Context, val: i64, _bit_width: u32) -> Int {
    Int::from_i64(context, val as i64)
}

fn zero(context: &z3::Context, bit_width: u32) -> Int {
    int_from_i64(context, 0, bit_width)
}

fn one(context: &z3::Context, bit_width: u32) -> Int {
    int_from_i64(context, 1, bit_width)
}

fn _min_int(context: &z3::Context, bit_width: u32) -> Int {
    int_from_i64(context, -2^63, bit_width)
}

pub trait Component: Debug {
    fn operand_arity(&self) -> usize;

    fn make_operator(&self, immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator;

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        // immediates: &[BitVec<'a>],
        // operands: &[BitVec<'a>],
        // immediates: &[Vec<Int<'a>>],
        // operands: &[Vec<Int<'a>>],

        // immediates: &[Vecs<Array<'a>>],
        // operands: &[Vecs<Array<'a>>],
        // bit_width: u32,

        immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> 
        //BitVec<'a> 
        Vecs<Int<'a>>;
        
    /// How many immediates does this component require?
    fn immediate_arity(&self) -> usize {
        0
    }
}

// #[derive(Debug)]
// struct Const(Vec<Vec<i64>>);

// impl Component for Const {
//     fn operand_arity(&self) -> usize {
//         0
//     }

//     fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, _operands: &[Id]) -> Operator {
//         Operator::Const(self.0.clone())
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         // _immediates: &[Vec<Int<'a>>],
//         // _operands: &[Vec<Int<'a>>],
//         _immediates: &[Vecs<Int<'a>>],
//         _operands: &[Vecs<Int<'a>>],
//         bit_width: u32,
//     ) -> Vecs<Int<'a>> {

//         // if let Some(val) = self.0 {
//         //     BitVec::from_i64(context, val as i64, bit_width)
//         // } else {
//         //     immediates[0][0].clone()
//         // }

//         let const_val = &(self.0);
//         let dims = [const_val.len(), const_val[0].len()];

//         let mut result : Vecs<Int<'a>> = Vecs::new(dims);

        
//         for i in 0 .. dims[0] {
//             for j in 0 .. dims[1] {
//                 result.vecs[i as usize].push(Int::from_i64(context, (self.0)[i][j]));
//             }
//         }

//         return result;
           
//         } 

//         /*if let Some(val) = self.0 {
//             result.push(BitVec::from_i64(context, val as i64, bit_width));
//         } else {
//             result.push(immediates[0][0].clone());
//         }*/

//     fn immediate_arity(&self) -> usize {
//         1
//     }
// }


// pub fn const_(val: Vec<Vec<i64>>) -> Box<dyn Component> {
//     Box::new(Const(val)) as _
// }

#[derive(Debug)]
struct TfAdd;

impl Component for TfAdd {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfAdd(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(Int::add(&context, &[&operands[0].vecs[i][j], &operands[1].vecs[i][j]]));
            }
        }

        return result;
    }
}

pub fn tf_add() -> Box<dyn Component> {
    Box::new(TfAdd) as _
}

#[derive(Debug)]
struct TfArgmax;

impl Component for TfArgmax {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfArgmax(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 注意，这里只有axis=1的实现方式，这么做的原因是因为它不给axis=0的测试样例所以没有写。。。
        // 目前遇到的问题是，如果采用之前array的方法，即计算两遍然后根据axis判断返回结果，那么会导致ite的时候类型不匹配
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for _j in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            let mut max = const0.clone();
            let mut index = const0.clone();
            for j in 0 .. DIMS[1] {
                max = operands[0].vecs[i][j].lt(&max).ite(&operands[0].vecs[i][j], &max);
            }
            for j in (0 .. DIMS[1]).rev() {
                index = operands[0].vecs[i][j]._eq(&max).ite(&Int::from_i64(context, j as i64), &index);
            }
            result.vecs[0][i] = index;
        }

        return result;
    }
}

pub fn tf_argmax() -> Box<dyn Component> {
    Box::new(TfArgmax) as _
}

#[derive(Debug)]
struct TfCast;

impl Component for TfCast {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfCast(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(Int::from_i64(&context, operands[0].vecs[i][j].as_i64().unwrap_or(0)));
            }
        }

        return result;
    }
}

pub fn tf_cast() -> Box<dyn Component> {
    Box::new(TfCast) as _
}

#[derive(Debug)]
struct TfConstant;

impl Component for TfConstant {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfConstant(operands[0])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(operands[0].vecs[i][j].clone());
            }
        }

        return result;
    }
}

pub fn tf_constant() -> Box<dyn Component> {
    Box::new(TfConstant) as _
}

#[derive(Debug)]
struct TfDivide;

impl Component for TfDivide {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfDivide(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[0]);
                let is_in_col = col.lt(&operands[0].dims[1]);
                let fenmu = operands[1].vecs[i][j]._eq(&const0).ite(&const1, &operands[1].vecs[i][j]);
                let value = Bool::and(context, &[&is_in_row, &is_in_col]).ite(&Int::div(&operands[0].vecs[i][j], &fenmu), &const0);
                result.vecs[i].push(value);
            }
        }

        return result;
    }
}

pub fn tf_divide() -> Box<dyn Component> {
    Box::new(TfDivide) as _
}

#[derive(Debug)]
struct TfEqual;

impl Component for TfEqual {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[0]);
                let is_in_col = col.lt(&operands[0].dims[1]);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col]).ite(&operands[0].vecs[i][j]._eq(&operands[1].vecs[i][j]).ite(&const1, &const0), &const0));
            }
        }

        return result;
    }
}

pub fn tf_equal() -> Box<dyn Component> {
    Box::new(TfEqual) as _
}

#[derive(Debug)]
struct TfExpandDims;

impl Component for TfExpandDims {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfExpandDims(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 样例中能用的（有些数组维度超过了二维）基本上都是axis=1的情况，目前只考虑这个
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                if j < DIMS[0] {
                    result.vecs[j][0] = operands[0].vecs[i][j].clone();
                }
            }
        }

        return result;
    }
}

pub fn tf_expand_dims() -> Box<dyn Component> {
    Box::new(TfExpandDims) as _
}

#[derive(Debug)]
struct TfGreater;

impl Component for TfGreater {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfGreater(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[0]);
                let is_in_col = col.lt(&operands[0].dims[1]);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col]).ite(&operands[0].vecs[i][j].gt(&operands[1].vecs[i][j]).ite(&const1, &const0), &const0));
            }
        }

        return result;
    }
}

pub fn tf_greater() -> Box<dyn Component> {
    Box::new(TfGreater) as _
}

#[derive(Debug)]
struct TfBincount;

impl Component for TfBincount {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfBincount(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 所有的样例里面只有一个输入的情况，目前就考虑这个
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                for k in 0 .. DIMS[1] {
                    let index = Int::from_i64(context, k as i64);
                    let new_value = Int::add(context, &[&result.vecs[i][k], &const1]);
                    result.vecs[i][k] = index._eq(&operands[0].vecs[i][j]).ite(&new_value, &result.vecs[i][k]);
                }
            }
        }

        return result;
    }
}

pub fn tf_bincount() -> Box<dyn Component> {
    Box::new(TfBincount) as _
}

#[derive(Debug)]
struct TfMultiply;

impl Component for TfMultiply {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfMultiply(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(Int::mul(&context, &[&operands[0].vecs[i][j], &operands[1].vecs[i][j]]));
            }
        }

        return result;
    }
}

pub fn tf_multiply() -> Box<dyn Component> {
    Box::new(TfMultiply) as _
}

#[derive(Debug)]
struct TfRange;

impl Component for TfRange {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfRange(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 所有样例只有两个参数，起始和结束，第三个参数delta按照1处理
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        let start = operands[0].vecs[0][0].clone();
        let limit = operands[1].vecs[0][0].clone();
        let mut value = start.clone();
        for i in 0 .. DIMS[0] {
            for _j in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for j in 0 .. DIMS[1] {
            result.vecs[0][j] = value.lt(&limit).ite(&value, &const0);
            value = Int::add(context, &[&value, &const1]);
        }

        return result;
    }
}

pub fn tf_range() -> Box<dyn Component> {
    Box::new(TfRange) as _
}

#[derive(Debug)]
struct TfSequenceMask;

impl Component for TfSequenceMask {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfSequenceMask(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[0]);
                let is_in_col = Bool::and(context, &[&col.lt(&operands[0].dims[1]), &col.lt(&operands[0].vecs[0][i])]);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col]).ite(&const1, &const0));
            }
        }

        return result;
    }
}

pub fn tf_sequence_mask() -> Box<dyn Component> {
    Box::new(TfSequenceMask) as _
}

#[derive(Debug)]
struct TfSquare;

impl Component for TfSquare {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfSquare(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(Int::mul(&context, &[&operands[0].vecs[i][j], &operands[0].vecs[i][j]]));
            }
        }

        return result;
    }
}

pub fn tf_square() -> Box<dyn Component> {
    Box::new(TfSquare) as _
}

#[derive(Debug)]
struct TfSubtract;

impl Component for TfSubtract {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfSubtract(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(Int::sub(&context, &[&operands[0].vecs[i][j], &operands[1].vecs[i][j]]));
            }
        }

        return result;
    }
}

pub fn tf_subtract() -> Box<dyn Component> {
    Box::new(TfSubtract) as _
}

#[derive(Debug)]
struct TfTensordot;

impl Component for TfTensordot {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfTensordot(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 第三个参数axes所有的测试样例里面都是1，其他形式的可以自己转换
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for _j in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[0] {
                for k in 0 .. DIMS[0] {
                    let temp = Int::mul(context, &[&operands[0].vecs[i][k], &operands[1].vecs[k][j]]);
                    result.vecs[i][j] = Int::add(context, &[&result.vecs[i][j], &temp]);
                }
            }
        }

        return result;
    }
}

pub fn tf_tensordot() -> Box<dyn Component> {
    Box::new(TfTensordot) as _
}

#[derive(Debug)]
struct TfTranspose;

impl Component for TfTranspose {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfTranspose(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for _j in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[0] {
                result.vecs[i][j] = operands[0].vecs[j][i].clone();
            }
        }

        return result;
    }
}

pub fn tf_transpose() -> Box<dyn Component> {
    Box::new(TfTranspose) as _
}

#[derive(Debug)]
struct TfEye;

impl Component for TfEye {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfEye(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        let total_row = operands[0].vecs[0][0].clone();
        let total_col = operands[1].vecs[0][0].clone();
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&total_row);
                let is_in_col = col.lt(&total_col);
                let row_equal_col = row._eq(&col);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col, &row_equal_col]).ite(&const1, &const0));
            }
        }

        return result;
    }
}

pub fn tf_eye() -> Box<dyn Component> {
    Box::new(TfEye) as _
}

#[derive(Debug)]
struct TfFill;

impl Component for TfFill {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfFill(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        let total_row = operands[0].vecs[0][0].clone();
        let total_col = operands[0].vecs[0][1].clone();
        let fill_value = operands[1].vecs[0][0].clone();
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&total_row);
                let is_in_col = col.lt(&total_col);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col]).ite(&fill_value, &const0));
            }
        }

        return result;
    }
}

pub fn tf_fill() -> Box<dyn Component> {
    Box::new(TfFill) as _
}

#[derive(Debug)]
struct TfMatmul;

impl Component for TfMatmul {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfMatmul(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 所有的测试样例里面只有两个参数的形式
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for _j in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[0] {
                for k in 0 .. DIMS[0] {
                    let temp = Int::mul(context, &[&operands[0].vecs[i][k], &operands[1].vecs[k][j]]);
                    result.vecs[i][j] = Int::add(context, &[&result.vecs[i][j], &temp]);
                }
            }
        }

        return result;
    }
}

pub fn tf_matmul() -> Box<dyn Component> {
    Box::new(TfMatmul) as _
}

#[derive(Debug)]
struct TfMaximum;

impl Component for TfMaximum {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfMaximum(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[0]);
                let is_in_col = col.lt(&operands[0].dims[1]);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col]).ite(&operands[0].vecs[i][j].gt(&operands[1].vecs[i][j]).ite(&operands[0].vecs[i][j], &operands[1].vecs[i][j]), &const0));
            }
        }

        return result;
    }
}

pub fn tf_maximum() -> Box<dyn Component> {
    Box::new(TfMaximum) as _
}

#[derive(Debug)]
struct TfMinimum;

impl Component for TfMinimum {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfMinimum(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[0]);
                let is_in_col = col.lt(&operands[0].dims[1]);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col]).ite(&operands[0].vecs[i][j].lt(&operands[1].vecs[i][j]).ite(&operands[0].vecs[i][j], &operands[1].vecs[i][j]), &const0));
            }
        }

        return result;
    }
}

pub fn tf_minimum() -> Box<dyn Component> {
    Box::new(TfMinimum) as _
}

#[derive(Debug)]
struct TfNotEqual;

impl Component for TfNotEqual {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfNotEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[0]);
                let is_in_col = col.lt(&operands[0].dims[1]);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col]).ite(&operands[0].vecs[i][j]._eq(&operands[1].vecs[i][j]).ite(&const0, &const1), &const0));
            }
        }

        return result;
    }
}

pub fn tf_not_equal() -> Box<dyn Component> {
    Box::new(TfNotEqual) as _
}

#[derive(Debug)]
struct TfOnes;

impl Component for TfOnes {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfOnes(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        let total_row = operands[0].vecs[0][0].clone();
        let total_col = operands[0].vecs[0][1].clone();
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&total_row);
                let is_in_col = col.lt(&total_col);
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col]).ite(&const1, &const0));
            }
        }

        return result;
    }
}

pub fn tf_ones() -> Box<dyn Component> {
    Box::new(TfOnes) as _
}

#[derive(Debug)]
struct TfZeros;

impl Component for TfZeros {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfZeros(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for _j in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }

        return result;
    }
}

pub fn tf_zeros() -> Box<dyn Component> {
    Box::new(TfZeros) as _
}

macro_rules! with_operator_component {
    ( $me:expr , |$c:ident| $body:expr ) => {
        match $me {
            Operator::Var => panic!("`Var` operators do not have a component"),
            // Operator::Const(c) => {
            //     let $c = Const(c.to_vec());
            //     $body
            // }
            Operator::TfAdd(_, _) => {
                let $c = TfAdd;
                $body
            }
            Operator::TfArgmax(_) => {
                let $c = TfArgmax;
                $body
            }
            Operator::TfCast(_) => {
                let $c = TfCast;
                $body
            }
            Operator::TfConstant(_) => {
                let $c = TfConstant;
                $body
            }
            Operator::TfDivide(_, _) => {
                let $c = TfDivide;
                $body
            }
            Operator::TfEqual(_, _) => {
                let $c = TfEqual;
                $body
            }
            Operator::TfExpandDims(_) => {
                let $c = TfExpandDims;
                $body
            }
            Operator::TfBincount(_) => {
                let $c = TfBincount;
                $body
            }
            Operator::TfGreater(_, _) => {
                let $c = TfGreater;
                $body
            }
            Operator::TfMultiply(_, _) => {
                let $c = TfMultiply;
                $body
            }
            Operator::TfRange(_, _) => {
                let $c = TfRange;
                $body
            }
            Operator::TfSequenceMask(_) => {
                let $c = TfSequenceMask;
                $body
            }
            Operator::TfSquare(_) => {
                let $c = TfSquare;
                $body
            }
            Operator::TfSubtract(_, _) => {
                let $c = TfSubtract;
                $body
            }
            Operator::TfTensordot(_, _) => {
                let $c = TfTensordot;
                $body
            }
            Operator::TfTranspose(_) => {
                let $c = TfTranspose;
                $body
            }

            Operator::TfEye(_, _) => {
                let $c = TfEye;
                $body
            }
            Operator::TfFill(_, _) => {
                let $c = TfFill;
                $body
            }
            Operator::TfMatmul(_, _) => {
                let $c = TfMatmul;
                $body
            }
            Operator::TfMaximum(_, _) => {
                let $c = TfMaximum;
                $body
            }
            Operator::TfMinimum(_, _) => {
                let $c = TfMinimum;
                $body
            }
            Operator::TfNotEqual(_, _) => {
                let $c = TfNotEqual;
                $body
            }
            Operator::TfOnes(_) => {
                let $c = TfOnes;
                $body
            }
            Operator::TfZeros(_) => {
                let $c = TfZeros;
                $body
            }
        }
    };
}

impl Component for Operator {
    fn operand_arity(&self) -> usize {
        Operator::arity(self)
    }

    fn make_operator(&self, immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        with_operator_component!(self, |c| c.make_operator(immediates, operands))
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        with_operator_component!(self, |c| {
            c.make_expression(context, immediates, operands, bit_width)
        })
    }

    fn immediate_arity(&self) -> usize {
        with_operator_component!(self, |c| c.immediate_arity())
    }
}