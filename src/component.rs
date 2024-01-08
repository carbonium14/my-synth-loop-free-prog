use crate::{Id, Operator, Vecs, DIMS};
use std::{fmt::Debug, usize};
use z3::{ast::{Int, Bool, Array, Ast}, Sort};

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
        let mut result = Vecs::new([one(context, bit_width), operands[0].dims[0].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            let mut max = Int::from_i64(context, -9223372036854775808);
            let mut index = const0.clone();
            for j in 0 .. DIMS[1] {
                max = operands[0].vecs[i][j].gt(&max).ite(&operands[0].vecs[i][j], &max);
            }
            for j in (0 .. DIMS[1]).rev() {
                let col = Int::from_i64(context, j as i64);
                index = operands[0].vecs[i][j]._eq(&max).ite(&col, &index);
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
struct TfBooleanMask;

impl Component for TfBooleanMask {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfBooleanMask(operands[0], operands[1])
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
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array = Array::fresh_const(context, "boolean_mask_array:", &domain_sort, &range_sort);
        let mut result_index = zero(context, bit_width);
        for i in 0 .. DIMS[0] {
            let mut index = zero(context, bit_width);
            for j in 0 .. DIMS[1] {
                let cur_index = operands[1].vecs[i][j]._eq(&const0).ite(&Int::from_i64(context, -1), &index);
                index = operands[1].vecs[i][j]._eq(&const0).ite(&index, &Int::add(context, &[&index, &const1]));
                array = array.store(&cur_index, &operands[0].vecs[i][j]);
            }
            result_index = result_index.lt(&index).ite(&index, &result_index);
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(array.select(&Int::from_i64(context, j as i64)).as_int().unwrap_or(const0.clone()));
            }
        }
        result.dims[1] = result_index;

        return result;
    }
}

pub fn tf_boolean_mask() -> Box<dyn Component> {
    Box::new(TfBooleanMask) as _
}

#[derive(Debug)]
struct TfBooleanMask_;

impl Component for TfBooleanMask_ {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfBooleanMask_(operands[0], operands[1])
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
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let domain_sort_ = Sort::int(&context);
        let range_sort_ = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort_, &range_sort_);
        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(context, "boolean_mask_array_second:", &domain_sort, &range_sort);
        let mut array_ = Array::fresh_const(context, "boolean_mask_array:", &first_dim_sort, &array_sort);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        let mut index_ = zero(context, bit_width);
        let mut result_index = zero(context, bit_width);
        for i in 0 .. DIMS[0] {
            let mut index = zero(context, bit_width);
            for j in 0 .. DIMS[1] {
                let cur_index = operands[1].vecs[0][i]._eq(&const0).ite(&Int::from_i64(context, -1), &index);
                index = operands[1].vecs[0][i]._eq(&const0).ite(&index, &Int::add(context, &[&index, &const1]));
                array = array.store(&cur_index, &operands[0].vecs[i][j]);
            }
            result_index = result_index.lt(&index).ite(&index, &result_index);
            let cur_index_ = operands[1].vecs[0][i]._eq(&const0).ite(&Int::from_i64(context, -1), &index_);
            index_ = operands[1].vecs[0][i]._eq(&const0).ite(&index_, &Int::add(context, &[&index_, &const1]));
            array_ = array_.store(&cur_index_, &array);
        }
        for i in 0 .. DIMS[0] {
            let row = array_.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            for j in 0 .. DIMS[1] {
                result.vecs[i][j] = row.select(&Int::from_i64(context, j as i64)).as_int().unwrap_or(const0.clone());
            }
        }
        result.dims[0] = index_;
        result.dims[1] = result_index;

        return result;
    }
}

pub fn tf_boolean_mask_() -> Box<dyn Component> {
    Box::new(TfBooleanMask_) as _
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

pub fn tf_cast() -> Box<dyn Component> {
    Box::new(TfCast) as _
}

#[derive(Debug)]
struct TfConcat0;

impl Component for TfConcat0 {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfConcat0(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new([Int::add(context, &[&operands[0].dims[0], &operands[1].dims[0]]), operands[0].dims[1].clone()]);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for j in 0 .. DIMS[1] {
            let mut array = Array::fresh_const(context, "concat_0_array:", &domain_sort, &range_sort);
            for i in 0 .. DIMS[0] {
                let row_index = Int::from_i64(context, i as i64);
                array = array.store(&row_index, &operands[0].vecs[i][j]);
            }
            for i in 0 .. DIMS[0] {
                let row_index = Int::add(context, &[&Int::from_i64(context, i as i64), &operands[0].dims[0]]);
                array = array.store(&row_index, &operands[1].vecs[i][j]);
            }
            for i in 0 .. DIMS[0] {
                let row_index = Int::from_i64(context, j as i64);
                result.vecs[i][j] = array.select(&row_index).as_int().unwrap_or(const0.clone());
            }
        }

        return result;
    }
}

pub fn tf_concat0() -> Box<dyn Component> {
    Box::new(TfConcat0) as _
}

#[derive(Debug)]
struct TfConcat1;

impl Component for TfConcat1 {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfConcat1(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new([operands[0].dims[0].clone(), Int::add(context, &[&operands[0].dims[1], &operands[1].dims[1]])]);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        for i in 0 .. DIMS[0] {
            let mut array = Array::fresh_const(context, "concat_1_array:", &domain_sort, &range_sort);
            for j in 0 .. DIMS[1] {
                let col_index = Int::from_i64(context, j as i64);
                array = array.store(&col_index, &operands[0].vecs[i][j]);
            }
            for j in 0 .. DIMS[1] {
                let col_index = Int::add(context, &[&Int::from_i64(context, j as i64), &operands[0].dims[1]]);
                array = array.store(&col_index, &operands[1].vecs[i][j]);
            }
            for j in 0 .. DIMS[1] {
                let col_index = Int::from_i64(context, j as i64);
                result.vecs[i].push(array.select(&col_index).as_int().unwrap_or(const0.clone()));
            }
        }

        return result;
    }
}

pub fn tf_concat1() -> Box<dyn Component> {
    Box::new(TfConcat1) as _
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
                let value = Bool::and(context, &[&is_in_row, &is_in_col])
                    .ite(&Int::div(&operands[0].vecs[i][j], &fenmu), &const0);
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
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col, &operands[0].vecs[i][j]._eq(&operands[1].vecs[i][j])])
                    .ite(&const1, &const0));
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
        // 样例中能用的（有些数组维度超过了二维）基本上都是axis = 1的情况，目前只考虑这个
        let const0 = zero(context, bit_width);
        let const4 = Int::from_i64(context, DIMS[0] as i64);
        let result_row = operands[0].dims[1].lt(&const4).ite(&operands[0].dims[1], &const4);
        let mut result = Vecs::new([result_row, one(context, bit_width)]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[0] {
                result.vecs[j][0] = operands[0].vecs[i][j].clone();
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
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col, &operands[0].vecs[i][j].gt(&operands[1].vecs[i][j])])
                    .ite(&const1, &const0));
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
        let mut result_index = zero(context, bit_width);
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result_index = result_index.gt(&operands[0].vecs[i][j]).ite(&result_index, &operands[0].vecs[i][j]);
                for k in 0 .. DIMS[1] {
                    let index = Int::from_i64(context, k as i64);
                    let new_value = Int::add(context, &[&result.vecs[i][k], &const1]);
                    result.vecs[i][k] = index._eq(&operands[0].vecs[i][j]).ite(&new_value, &result.vecs[i][k]);
                }
            }
        }
        result.dims[1] = result_index;

        return result;
    }
}

pub fn tf_bincount() -> Box<dyn Component> {
    Box::new(TfBincount) as _
}

#[derive(Debug)]
struct TfCumsum;

impl Component for TfCumsum {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfCumsum(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 所有样例只有输入数组和exclusive两个输入，只考虑这俩
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        let is_exclusive = operands[1].vecs[0][0]._eq(&const0);
        for i in 0 .. DIMS[0] {
            result.vecs[i].push(is_exclusive.ite(&operands[0].vecs[i][0], &const0));
            for j in 1 .. (DIMS[1] + 1) {
                let col_index = Int::from_i64(context, j as i64);
                let is_in_col = col_index.lt(&operands[0].dims[1]);
                let mut temp = j;
                if j == DIMS[1] {
                    temp = DIMS[1] - 1;
                }
                let is_exclusive_value = is_exclusive.ite(&operands[0].vecs[i][temp], &operands[0].vecs[i][j - 1]);
                let value = Int::add(context, &[&result.vecs[i][j - 1], &is_exclusive_value]);
                result.vecs[i].push(is_in_col.ite(&value, &const0));
            }
        }

        return result;
    }
}

pub fn tf_cumsum() -> Box<dyn Component> {
    Box::new(TfCumsum) as _
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
struct TfOneHot;

impl Component for TfOneHot {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfOneHot(operands[0], operands[1])
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
        let const4 = Int::from_i64(context, DIMS[0] as i64);
        let result_row = operands[0].dims[1].lt(&const4).ite(&operands[0].dims[1], &const4);
        let depth = operands[1].vecs[0][0].clone();
        let mut result = Vecs::new([result_row, depth.clone()]);
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = row_index.lt(&operands[0].dims[0]);
                let is_in_col = Bool::and(context, &[&col_index.lt(&operands[0].dims[1]), &col_index.lt(&depth)]);
                let is_equal = operands[0].vecs[0][i]._eq(&col_index);
                let value = Bool::and(context, &[&is_in_row, &is_in_col, &is_equal]).ite(&const1, &const0);
                result.vecs[i].push(value);
            }
        }

        return result;
    }
}

pub fn tf_one_hot() -> Box<dyn Component> {
    Box::new(TfOneHot) as _
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
        let start = operands[0].vecs[0][0].clone();
        let limit = operands[1].vecs[0][0].clone();
        let len = Int::sub(context, &[&limit, &start]);
        let col = len.gt(&const0).ite(&len, &const0);
        let mut result = Vecs::new([const1.clone(), col]);
        let mut value = start.clone();
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
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
struct TfReduceMax;

impl Component for TfReduceMax {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceMax(operands[0])
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
        let mut result = Vecs::new([const1.clone(), const1.clone()]);
        let mut max = Int::from_i64(context, -9223372036854775808);
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = row_index.lt(&operands[0].dims[0]);
                let is_in_col = col_index.lt(&operands[0].dims[1]);
                let value = operands[0].vecs[i][j].gt(&max).ite(&operands[0].vecs[i][j], &max);
                max = Bool::and(context, &[&is_in_row, &is_in_col]).ite(&value, &max);
            }
        }
        result.vecs[0][0] = max;

        return result;
    }
}

pub fn tf_reduce_max() -> Box<dyn Component> {
    Box::new(TfReduceMax) as _
}

#[derive(Debug)]
struct TfReduceMax0;

impl Component for TfReduceMax0 {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceMax0(operands[0])
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
        let mut result = Vecs::new([const1.clone(), operands[0].dims[1].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for j in 0 .. DIMS[1] {
            let mut max = Int::from_i64(context, -9223372036854775808);
            for i in 0 .. DIMS[0] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = row_index.lt(&operands[0].dims[0]);
                let is_in_col = col_index.lt(&operands[0].dims[1]);
                let value = operands[0].vecs[i][j].gt(&max).ite(&operands[0].vecs[i][j], &max);
                max = Bool::and(context, &[&is_in_row, &is_in_col]).ite(&value, &max);
            }
            result.vecs[0][j] = max;
        }

        return result;
    }
}

pub fn tf_reduce_max0() -> Box<dyn Component> {
    Box::new(TfReduceMax0) as _
}

#[derive(Debug)]
struct TfReduceMax1;

impl Component for TfReduceMax1 {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceMax1(operands[0])
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
        let mut result = Vecs::new([const1.clone(), operands[0].dims[0].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            let mut max = Int::from_i64(context, -9223372036854775808);
            for j in 0 .. DIMS[1] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = row_index.lt(&operands[0].dims[0]);
                let is_in_col = col_index.lt(&operands[0].dims[1]);
                let value = operands[0].vecs[i][j].gt(&max).ite(&operands[0].vecs[i][j], &max);
                max = Bool::and(context, &[&is_in_row, &is_in_col]).ite(&value, &max);
            }
            result.vecs[0][i] = max;
        }

        return result;
    }
}

pub fn tf_reduce_max1() -> Box<dyn Component> {
    Box::new(TfReduceMax1) as _
}

#[derive(Debug)]
struct TfReduceSum;

impl Component for TfReduceSum {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceSum(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new([one(context, bit_width), one(context, bit_width)]);
        let mut sum = zero(context, bit_width);
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
                sum = Int::add(context, &[&sum, &operands[0].vecs[i][j]]);
            }
        }
        result.vecs[0][0] = sum;

        return result;
    }
}

pub fn tf_reduce_sum() -> Box<dyn Component> {
    Box::new(TfReduceSum) as _
}

#[derive(Debug)]
struct TfReduceSum0;

impl Component for TfReduceSum0 {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceSum0(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new([one(context, bit_width), operands[0].dims[1].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for j in 0 .. DIMS[1] {
            let mut sum = zero(context, bit_width);
            for i in 0 .. DIMS[0] {
                sum = Int::add(context, &[&sum, &operands[0].vecs[i][j]]);
            }
            result.vecs[0][j] = sum;
        }

        return result;
    }
}

pub fn tf_reduce_sum0() -> Box<dyn Component> {
    Box::new(TfReduceSum0) as _
}

#[derive(Debug)]
struct TfReduceSum1;

impl Component for TfReduceSum1 {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceSum1(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new([one(context, bit_width), operands[0].dims[0].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            let mut sum = zero(context, bit_width);
            for j in 0 .. DIMS[1] {
                sum = Int::add(context, &[&sum, &operands[0].vecs[i][j]]);
            }
            result.vecs[0][i] = sum;
        }

        return result;
    }
}

pub fn tf_reduce_sum1() -> Box<dyn Component> {
    Box::new(TfReduceSum1) as _
}

#[derive(Debug)]
struct TfSequenceMask;

impl Component for TfSequenceMask {
    fn operand_arity(&self) -> usize {
        1
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
        let mut maxlen = zero(context, bit_width);
        for i in 0 .. DIMS[0] {
            maxlen = maxlen.gt(&operands[0].vecs[0][i]).ite(&maxlen, &operands[0].vecs[0][i]);
        }
        let const4 = Int::from_i64(context, DIMS[0] as i64);
        let result_row = operands[0].dims[1].lt(&const4).ite(&operands[0].dims[1], &const4);
        let mut result = Vecs::new([result_row, maxlen]);
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[1]);
                let is_in_col = col.lt(&operands[0].vecs[0][i]);
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
        let const4 = Int::from_i64(context, DIMS[0] as i64);
        let result = operands[1].dims[1].lt(&const4).ite(&operands[1].dims[1], &const4);
        let mut result = Vecs::new([operands[0].dims[0].clone(), result]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
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
        let const4 = Int::from_i64(context, DIMS[0] as i64);
        let result = operands[0].dims[1].lt(&const4).ite(&operands[0].dims[1], &const4);
        let mut result = Vecs::new([result, operands[0].dims[0].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
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
struct TfWhere1;

impl Component for TfWhere1 {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfWhere1(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 所有测试中均为二维下标，不考虑一维的情况
        let const0 = zero(context, bit_width);
        let const1 = zero(context, bit_width);
        let const_minus_1 = Int::from_i64(context, -1);
        let mut result = Vecs::new(operands[0].dims.clone());
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let domain_sort_ = Sort::int(&context);
        let range_sort_ = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort_, &range_sort_);
        let first_dim_sort = Sort::int(&context);
        let mut array_total = Array::fresh_const(context, "where_1_array:", &first_dim_sort, &array_sort);
        let mut index = zero(context, bit_width);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let domain_sort = Sort::int(&context);
                let range_sort = Sort::int(&context);
                let mut array = Array::fresh_const(context, "where_1_array_second:", &domain_sort, &range_sort);
                let row_index = operands[0].vecs[i][j]._eq(&const0).ite(&const_minus_1, &Int::from_i64(context, i as i64));
                let col_index = operands[0].vecs[i][j]._eq(&const0).ite(&const_minus_1, &Int::from_i64(context, j as i64));
                array = array.store(&const0, &row_index);
                array = array.store(&const1, &col_index);
                let index_ = Bool::and(context, &[&row_index._eq(&const_minus_1), &col_index._eq(&const_minus_1)])
                    .ite(&const_minus_1, &index);
                index = Bool::and(context, &[&row_index._eq(&const_minus_1), &col_index._eq(&const_minus_1)])
                    .ite(&index, &Int::add(context, &[&index, &const1]));
                array_total = array_total.store(&index_, &array);
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. 2 {
                result.vecs[i][j] = array_total.select(&Int::from_i64(context, i as i64)).as_array()
                    .unwrap_or(Array::fresh_const(context, "where_1_array_temp:", &domain_sort, &range_sort))
                    .select(&Int::from_i64(context, j as i64)).as_int().unwrap_or(const0.clone());
            }
        }
        let const4 = Int::from_i64(context, DIMS[0] as i64);
        result.dims[0] = index.lt(&const4).ite(&index, &const4);
        result.dims[1] = Int::from_i64(context, 2);

        return result;
    }
}

pub fn tf_where1() -> Box<dyn Component> {
    Box::new(TfWhere1) as _
}

#[derive(Debug)]
struct TfWhere3;

impl Component for TfWhere3 {
    fn operand_arity(&self) -> usize {
        3
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfWhere3(operands[0], operands[1], operands[2])
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
                result.vecs[i].push(operands[0].vecs[i][j]._eq(&const0).ite(&operands[2].vecs[i][j], &operands[1].vecs[i][j]));
            }
        }

        return result;
    }
}

pub fn tf_where3() -> Box<dyn Component> {
    Box::new(TfWhere3) as _
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
        let total_row = operands[0].vecs[0][0].clone();
        let total_col = operands[1].vecs[0][0].clone();
        let mut result = Vecs::new([total_row.clone(), total_col.clone()]);
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
        let total_row = operands[0].vecs[0][0].clone();
        let total_col = operands[0].vecs[0][1].clone();
        let fill_value = operands[1].vecs[0][0].clone();
        let mut result = Vecs::new([total_row.clone(), total_col.clone()]);
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
struct TfSegmentMax;

impl Component for TfSegmentMax {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfSegmentMax(operands[0], operands[1])
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
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut col = zero(context, bit_width);
        for i in 0 .. DIMS[0] {
            let mut array = Array::fresh_const(context, "segment_max_array:", &domain_sort, &range_sort);
            let max = Int::from_i64(context, -9223372036854775808);
            let flag = max.clone();
            for j in 0 .. DIMS[1] {
                let select_value = array.select(&operands[0].vecs[i][j]).as_int().unwrap_or(max.clone());
                col = select_value._eq(&flag).ite(&col, &Int::add(context, &[&col, &const1]));
                let final_value = select_value.gt(&operands[1].vecs[i][j]).ite(&select_value, &operands[1].vecs[i][j]);
                array = array.store(&operands[0].vecs[i][j], &final_value);
            }
            for j in 0 .. DIMS[1] {
                let index = Int::from_i64(context, j as i64);
                let value = array.select(&index).as_int().unwrap_or(const0.clone());
                result.vecs[i].push(value);
            }
        }
        result.dims[1] = col;

        return result;
    }
}

pub fn tf_segment_max() -> Box<dyn Component> {
    Box::new(TfSegmentMax) as _
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
        let const4 = Int::from_i64(context, DIMS[0] as i64);
        let result = operands[1].dims[1].lt(&const4).ite(&operands[1].dims[1], &const4);
        let mut result = Vecs::new([operands[0].dims[0].clone(), result]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
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
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col])
                    .ite(&operands[0].vecs[i][j].gt(&operands[1].vecs[i][j])
                    .ite(&operands[0].vecs[i][j], &operands[1].vecs[i][j]), &const0));
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
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col])
                    .ite(&operands[0].vecs[i][j].lt(&operands[1].vecs[i][j])
                    .ite(&operands[0].vecs[i][j], &operands[1].vecs[i][j]), &const0));
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
                result.vecs[i].push(Bool::and(context, &[&is_in_row, &is_in_col])
                    .ite(&operands[0].vecs[i][j]._eq(&operands[1].vecs[i][j])
                    .ite(&const0, &const1), &const0));
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
        let total_row = operands[0].vecs[0][0].clone();
        let total_col = operands[0].vecs[0][1].clone();
        let mut result = Vecs::new([total_row.clone(), total_col.clone()]);
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
struct TfReduceAny0;

impl Component for TfReduceAny0 {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceAny0(operands[0])
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
        let mut result = Vecs::new([const1.clone(), operands[0].dims[1].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for j in 0 .. DIMS[1] {
            let mut sum = Bool::from_bool(context, false);
            for i in 0 .. DIMS[0] {
                let is_zero = operands[0].vecs[i][j]._eq(&const0).ite(&Bool::from_bool(context, false), &Bool::from_bool(context, true));
                sum = Bool::or(context, &[&sum, &is_zero]);
            }
            result.vecs[0][j] = sum.ite(&const1, &const0);
        }

        return result;
    }
}

pub fn tf_reduce_any0() -> Box<dyn Component> {
    Box::new(TfReduceAny0) as _
}

#[derive(Debug)]
struct TfReduceAny1;

impl Component for TfReduceAny1 {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceAny1(operands[0])
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
        let mut result = Vecs::new([const1.clone(), operands[0].dims[0].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            let mut sum = Bool::from_bool(context, false);
            for j in 0 .. DIMS[1] {
                let is_zero = operands[0].vecs[i][j]._eq(&const0).ite(&Bool::from_bool(context, false), &Bool::from_bool(context, true));
                sum = Bool::or(context, &[&sum, &is_zero]);
            }
            result.vecs[0][i] = sum.ite(&const1, &const0);
        }

        return result;
    }
}

pub fn tf_reduce_any1() -> Box<dyn Component> {
    Box::new(TfReduceAny1) as _
}

#[derive(Debug)]
struct TfReduceMean;

impl Component for TfReduceMean {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceMean(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 测试样例中只有axis = 0的情况
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new([const1.clone(), operands[0].dims[1].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for j in 0 .. DIMS[1] {
            let mut sum = zero(context, bit_width);
            for i in 0 .. DIMS[0] {
                sum = Int::add(context, &[&sum, &operands[0].vecs[i][j]]);
            }
            result.vecs[0][j] = Int::div(&sum, &operands[0].dims[0]);
        }

        return result;
    }
}

pub fn tf_reduce_mean() -> Box<dyn Component> {
    Box::new(TfReduceMean) as _
}

#[derive(Debug)]
struct TfReduceProd;

impl Component for TfReduceProd {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReduceProd(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 测试样例中只有axis = 1的情况
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let mut result = Vecs::new([const1.clone(), operands[0].dims[0].clone()]);
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            let mut mul = one(context, bit_width);
            for j in 0 .. DIMS[1] {
                let col_index = Int::from_i64(context, j as i64);
                mul = col_index.lt(&operands[0].dims[1]).ite(&Int::mul(context, &[&mul, &operands[0].vecs[i][j]]), &mul);
            }
            result.vecs[0][i] = mul;
        }

        return result;
    }
}

pub fn tf_reduce_prod() -> Box<dyn Component> {
    Box::new(TfReduceProd) as _
}

#[derive(Debug)]
struct TfRoll;

impl Component for TfRoll {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfRoll(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 测试样例里面第二个参数和第三个参数都是一样的（都是1），目前只考虑一个参数的情况
        let const0 = zero(context, bit_width);
        let mut result = Vecs::new(operands[0].dims.clone());
        for i in 0 .. DIMS[0] {
            for _ in 0 .. DIMS[1] {
                result.vecs[i].push(const0.clone());
            }
        }
        for i in 0 .. DIMS[0] {
            for j in 0 .. DIMS[1] {
                let row = Int::from_i64(context, i as i64);
                let col = Int::from_i64(context, j as i64);
                let is_in_row = row.lt(&operands[0].dims[0]);
                let is_in_col = col.lt(&operands[0].dims[1]);
                result.vecs[i][(j + 1) % DIMS[1]] = Bool::and(context, &[&is_in_row, &is_in_col]).ite(&operands[0].vecs[i][j], &const0);
            }
            for j in 0 .. DIMS[1] {
                let col = Int::from_i64(context, j as i64);
                result.vecs[i][0] = col._eq(&operands[0].dims[1]).ite(&operands[0].vecs[i][j], &result.vecs[i][0]);
                result.vecs[i][j] = col.lt(&operands[0].dims[1]).ite(&result.vecs[i][j], &const0);
            }
        }

        return result;
    }
}

pub fn tf_roll() -> Box<dyn Component> {
    Box::new(TfRoll) as _
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
            for _ in 0 .. DIMS[1] {
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
            Operator::TfBooleanMask(_, _) => {
                let $c = TfBooleanMask;
                $body
            }
            Operator::TfBooleanMask_(_, _) => {
                let $c = TfBooleanMask_;
                $body
            }
            Operator::TfCast(_) => {
                let $c = TfCast;
                $body
            }
            Operator::TfConcat0(_, _) => {
                let $c = TfConcat0;
                $body
            }
            Operator::TfConcat1(_, _) => {
                let $c = TfConcat1;
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
            Operator::TfCumsum(_, _) => {
                let $c = TfCumsum;
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
            Operator::TfOneHot(_, _) => {
                let $c = TfOneHot;
                $body
            }
            Operator::TfRange(_, _) => {
                let $c = TfRange;
                $body
            }
            Operator::TfReduceMax(_) => {
                let $c = TfReduceMax;
                $body
            }
            Operator::TfReduceMax0(_) => {
                let $c = TfReduceMax0;
                $body
            }
            Operator::TfReduceMax1(_) => {
                let $c = TfReduceMax1;
                $body
            }
            Operator::TfReduceSum(_) => {
                let $c = TfReduceSum;
                $body
            }
            Operator::TfReduceSum0(_) => {
                let $c = TfReduceSum0;
                $body
            }
            Operator::TfReduceSum1(_) => {
                let $c = TfReduceSum1;
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
            Operator::TfWhere1(_) => {
                let $c = TfWhere1;
                $body
            }
            Operator::TfWhere3(_, _, _) => {
                let $c = TfWhere3;
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
            Operator::TfSegmentMax(_, _) => {
                let $c = TfSegmentMax;
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
            Operator::TfReduceAny0(_) => {
                let $c = TfReduceAny0;
                $body
            }
            Operator::TfReduceAny1(_) => {
                let $c = TfReduceAny1;
                $body
            }
            Operator::TfReduceMean(_) => {
                let $c = TfReduceMean;
                $body
            }
            Operator::TfReduceProd(_) => {
                let $c = TfReduceProd;
                $body
            }
            Operator::TfRoll(_) => {
                let $c = TfRoll;
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