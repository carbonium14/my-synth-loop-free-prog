use crate::{Id, Operator, Vecs};
use std::{fmt::Debug, ops::Sub, usize};
use z3::ast::{Ast, Int, Array, Bool};

use z3::Sort;

const DIMSIZE : [usize ; 2] = [4, 10];
const SIZE_STORE_INDEX : i64 = -2;
const SIZE_X : i64 = 0;
const SIZE_Y : i64 = 1;

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

    fn make_operator(&self, immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator;

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        // immediates: &[BitVec<'a>],
        // operands: &[BitVec<'a>],
        // immediates: &[Vec<Int<'a>>],
        // operands: &[Vec<Int<'a>>],

        immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> 
        //BitVec<'a> 
        Vecs<Array<'a>>;
        
    /// How many immediates does this component require?
    fn immediate_arity(&self) -> usize {
        0
    }
}

/*#[derive(Debug)]
struct Const(Vec<Vec<i64>>);

impl Component for Const {
    fn operand_arity(&self) -> usize {
        0
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, _operands: &[Id]) -> Operator {
        Operator::Const(self.0.clone())
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        // _immediates: &[Vec<Int<'a>>],
        // _operands: &[Vec<Int<'a>>],
        _immediates: &[Vecs<Int<'a>>],
        _operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {

        // if let Some(val) = self.0 {
        //     BitVec::from_i64(context, val as i64, bit_width)
        // } else {
        //     immediates[0][0].clone()
        // }

        let const_val = &(self.0);
        let dims = [const_val.len(), const_val[0].len()];

        let mut result : Vecs<Int<'a>> = Vecs::new(dims);

        
        for i in 0 .. dims[0] {
            for j in 0 .. dims[1] {
                result.vecs[i as usize].push(Int::from_i64(context, (self.0)[i][j]));
            }
        }

        return result;
           
        } 

        /*if let Some(val) = self.0 {
            result.push(BitVec::from_i64(context, val as i64, bit_width));
        } else {
            result.push(immediates[0][0].clone());
        }*/

    fn immediate_arity(&self) -> usize {
        1
    }
}


pub fn const_(val: Vec<Vec<i64>>) -> Box<dyn Component> {
    Box::new(Const(val)) as _
}
*/

#[derive(Debug)]
struct TfAbs;

impl Component for TfAbs {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfAbs(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {

        //TODO：目前只是二维,对于一维的数组，我们用x[1][m]表示长度为m的一维数组，他的dims = [1,m]

        // 取相同长度并且填充为0的数组，作取相反数的结果用
        let const0 = zero(context, bit_width);
        let op_arr = &operands[0].vecs;
        
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "abs_array", &first_dim_sort, &array_sort);
    

        // 它（for循环）是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
        // 所以这里是左闭右开区间
        for i in 0..DIMSIZE[0] {
            let now_index = Int::from_i64(context, i as i64);

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "abs_array_second:", &domain_sort, &range_sort);

            let now_arr = op_arr.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            for j in 0..DIMSIZE[1] {
                let index = Int::from_i64(context, j as i64);
                let m: Int<'_> = now_arr.select(&index).as_int().unwrap();

                array_val = array_val.store(&index ,&(m.lt(&const0).ite(&(const0.clone().sub(&m)), &m)));
            } 
            array = array.store(&now_index, &array_val);
        }

        //abs对于数组的维度不会产生变化，直接获取即可
        let size_info = operands[0].vecs.select(&Int::from_i64(&context, SIZE_STORE_INDEX)).as_array().unwrap();
        array = array.store(&Int::from_i64(&context, SIZE_STORE_INDEX), &size_info);

        let result: Vecs<Array> = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_abs() -> Box<dyn Component> {
    Box::new(TfAbs) as _
}

#[derive(Debug)]
struct TfAdd;

impl Component for TfAdd {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfAdd(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        _bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "add_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "add_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::add(&context, &[&in1_value_i_j, &in2_value_i_j]);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_add() -> Box<dyn Component> {
    Box::new(TfAdd) as _
}

#[derive(Debug)]
struct TfMul;

impl Component for TfMul {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfMul(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        _bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "mul_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "mul_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::mul(&context, &[&in1_value_i_j, &in2_value_i_j]);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_mul() -> Box<dyn Component> {
    Box::new(TfMul) as _
}

#[derive(Debug)]
struct TfDiv;

impl Component for TfDiv {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfDiv(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "div_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "div_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                // 要注意分母不能是0
                let div_value = Int::_eq(&in2_value_i_j, &const0).ite(&const1, &in2_value_i_j);
                let value = Int::div(&in1_value_i_j, &div_value);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_div() -> Box<dyn Component> {
    Box::new(TfDiv) as _
}

#[derive(Debug)]
struct TfArgMax;

impl Component for TfArgMax {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfArgMax(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "argmax_array_axis_1", &first_dim_sort, &array_sort);

        let domain_sort_ = Sort::int(&context);
        let range_sort_ = Sort::int(&context);
        let array_sort_ = Sort::array(context, &domain_sort_, &range_sort_);

        let first_dim_sort_ = Sort::int(&context);
        let mut array_ = Array::fresh_const(&context,  "argmax_array_axis_0", &first_dim_sort_, &array_sort_);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let in1_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let in1_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let axis = operands[1].vecs.select(&const0).as_array().unwrap().select(&const0).as_int().unwrap();
        // 注意维度是一个一维数组，长度是数入数组的第一维度
        let result_size_x = Int::from_i64(&context, 1);
        let result_size_y_0 = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let result_size_y_1 = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = axis._eq(&const0).ite(&result_size_y_0, &result_size_y_1);

        let domain_sort_1 = Sort::int(&context);
        let range_sort_1 = Sort::int(&context);
        let mut array_val_1 = Array::fresh_const(context, "argmax_array_second_axis_1:", &domain_sort_1, &range_sort_1);

        let domain_sort_1_ = Sort::int(&context);
        let range_sort_1_ = Sort::int(&context);
        let mut array_val_1_ = Array::fresh_const(context, "argmax_array_second_axis_0:", &domain_sort_1_, &range_sort_1_);

        for i in 0 .. DIMSIZE[0] {
            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            // 记录最大值
            let mut value = zero(context, bit_width);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &in1_size_x);
                let is_in_col = Int::lt(&col_index, &in1_size_y);
                // 在范围内则依次比较，找到最大值
                value = is_in_row.ite(&is_in_col.ite(&Int::gt(&in1_value_i_j, &value).ite(&in1_value_i_j, &value), &value), &value);
            }
            // 记录最大值对应的下标
            let mut res = zero(context, bit_width);
            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                // 在范围内则找到最大值对应的下标
                res = is_in_row.ite(&is_in_col.ite(&Int::_eq(&value, &in1_value_i_j).ite(&col_index, &res), &res), &res);
            }
            // 找到最终结果之后再放入
            array_val_1 = array_val_1.store(&Int::from_i64(context, i as i64), &res);
        }

        array = array.store(&Int::from_i64(context, 0), &array_val_1);

        for i in 0 .. DIMSIZE[1] {
            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            // 记录最大值
            let mut value = zero(context, bit_width);

            for j in 0 .. DIMSIZE[0] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, j as i64);
                let col_index = Int::from_i64(context, i as i64);
                let is_in_row = Int::lt(&row_index, &in1_size_x);
                let is_in_col = Int::lt(&col_index, &in1_size_y);
                // 在范围内则依次比较，找到最大值
                value = is_in_row.ite(&is_in_col.ite(&Int::gt(&in1_value_i_j, &value).ite(&in1_value_i_j, &value), &value), &value);
            }
            // 记录最大值对应的下标
            let mut res = zero(context, bit_width);
            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                // 在范围内则找到最大值对应的下标
                res = is_in_row.ite(&is_in_col.ite(&Int::_eq(&value, &in1_value_i_j).ite(&col_index, &res), &res), &res);
            }
            // 找到最终结果之后再放入
            array_val_1_ = array_val_1_.store(&Int::from_i64(context, i as i64), &res);
        }

        array_ = array_.store(&Int::from_i64(context, 0), &array_val_1_);

        array = axis._eq(&const0).ite(&array_, &array);

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_argmax() -> Box<dyn Component> {
    Box::new(TfArgMax) as _
}

#[derive(Debug)]
struct TfArgMin;

impl Component for TfArgMin {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfArgMin(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "argmax_array_axis_1", &first_dim_sort, &array_sort);

        let domain_sort_ = Sort::int(&context);
        let range_sort_ = Sort::int(&context);
        let array_sort_ = Sort::array(context, &domain_sort_, &range_sort_);

        let first_dim_sort_ = Sort::int(&context);
        let mut array_ = Array::fresh_const(&context,  "argmax_array_axis_0", &first_dim_sort_, &array_sort_);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let in1_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let in1_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let axis = operands[1].vecs.select(&const0).as_array().unwrap().select(&const0).as_int().unwrap();
        // 注意维度是一个一维数组，长度是数入数组的第一维度
        let result_size_x = Int::from_i64(&context, 1);
        let result_size_y_0 = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let result_size_y_1 = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = axis._eq(&const0).ite(&result_size_y_0, &result_size_y_1);

        let domain_sort_1 = Sort::int(&context);
        let range_sort_1 = Sort::int(&context);
        let mut array_val_1 = Array::fresh_const(context, "argmax_array_second_axis_1:", &domain_sort_1, &range_sort_1);

        let domain_sort_1_ = Sort::int(&context);
        let range_sort_1_ = Sort::int(&context);
        let mut array_val_1_ = Array::fresh_const(context, "argmax_array_second_axis_0:", &domain_sort_1_, &range_sort_1_);

        for i in 0 .. DIMSIZE[0] {
            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            // 记录最小值
            let mut value = Int::from_i64(&context, 9223372036854775807);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &in1_size_x);
                let is_in_col = Int::lt(&col_index, &in1_size_y);
                // 在范围内则依次比较，找到最小值
                value = is_in_row.ite(&is_in_col.ite(&Int::lt(&in1_value_i_j, &value).ite(&in1_value_i_j, &value), &value), &value);
            }
            // 记录最小值对应的下标
            let mut res = zero(context, bit_width);
            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                // 在范围内则找到最小值对应的下标
                res = is_in_row.ite(&is_in_col.ite(&Int::_eq(&value, &in1_value_i_j).ite(&col_index, &res), &res), &res);
            }
            // 找到最终结果之后再放入
            array_val_1 = array_val_1.store(&Int::from_i64(context, i as i64), &res);
        }

        array = array.store(&Int::from_i64(context, 0), &array_val_1);

        for i in 0 .. DIMSIZE[1] {
            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            // 记录最小值
            let mut value = zero(context, bit_width);

            for j in 0 .. DIMSIZE[0] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, j as i64);
                let col_index = Int::from_i64(context, i as i64);
                let is_in_row = Int::lt(&row_index, &in1_size_x);
                let is_in_col = Int::lt(&col_index, &in1_size_y);
                // 在范围内则依次比较，找到最小值
                value = is_in_row.ite(&is_in_col.ite(&Int::gt(&in1_value_i_j, &value).ite(&in1_value_i_j, &value), &value), &value);
            }
            // 记录最小值对应的下标
            let mut res = zero(context, bit_width);
            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                // 在范围内则找到最小值对应的下标
                res = is_in_row.ite(&is_in_col.ite(&Int::_eq(&value, &in1_value_i_j).ite(&col_index, &res), &res), &res);
            }
            // 找到最终结果之后再放入
            array_val_1_ = array_val_1_.store(&Int::from_i64(context, i as i64), &res);
        }

        array_ = array_.store(&Int::from_i64(context, 0), &array_val_1_);

        array = axis._eq(&const0).ite(&array_, &array);

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_argmin() -> Box<dyn Component> {
    Box::new(TfArgMin) as _
}

#[derive(Debug)]
struct TfBooleanMask;

impl Component for TfBooleanMask {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfBooleanMask(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        // 获取两个输入的长度，等长后进行后续操作
        // mask（也就是第二个参数）的维度可以是和数入数组等维度，也可以是维度少一维
        // 填充物，如果掩码部分不是1，那么就返回0
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let const_mins_1 = Int::from_i64(context, -1);
        //let const_min = min_int(context, bit_width);
        // 保证两个长度相等，否则报错

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "boolean_mask_array", &first_dim_sort, &array_sort);

        //先把mask的size取出来
        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let value_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let value_size_x = value_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let mask_size = operands[1].vecs.select(&size_index).as_array().unwrap();
        let mask_size_x = mask_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let _mask_size_y = mask_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        //result的size
        let mut result_size_x = value_size_x.clone();
        let mut result_size_y = const0.clone();

        //to value是1*多少的数组，mask也是1*多少的数组

        let mask_value_0 = operands[1].vecs.select(&Int::from_i64(context, 0 as i64)).as_array().unwrap();

        for i in 0 .. DIMSIZE[0] {
            let mut index_result = Int::from_i64(context, 0);

            let operand_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let mask_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "boolean_mask_array_second:", &domain_sort, &range_sort);

            let mask_value_0_i = mask_value_0.select(&Int::from_i64(context, i as i64)).as_int().unwrap();

            for j in 0 .. DIMSIZE[1] {
                //对于操作数的[i][j]
                let mask_value_i_j = mask_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let operand_i_j = operand_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();                

                //采用mask_value_0_i的情况：mask_size_x == 1 and value_size_x > 1
                
                let mask_judge = Bool::and(&context, &[&mask_size_x._eq(&const1), &value_size_x.gt(&const1)]).ite(&mask_value_0_i, &mask_value_i_j);
                let temp_index = mask_judge._eq(&const0).ite(&const_mins_1, &index_result);
               
                array_val = array_val.store(&temp_index, &operand_i_j); 
                
                index_result = temp_index._eq(&const_mins_1).ite(&index_result, &(Int::add(context, &[&index_result, &const1])));
            }
            result_size_y = index_result.clone();
            result_size_x = index_result._eq(&const0).ite(&(Int::sub(context, &[&result_size_x, &const1])), &result_size_x);

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        //将真实size存入
        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_boolean_mask() -> Box<dyn Component> {
    Box::new(TfBooleanMask) as _
}

#[derive(Debug)]
struct TfCast;

impl Component for TfCast {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfCast(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        _bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "cast_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "cast_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::from_i64(context, in1_value_i_j.as_i64().unwrap());
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_cast() -> Box<dyn Component> {
    Box::new(TfCast) as _
}

#[derive(Debug)]
struct TfConcat;

impl Component for TfConcat {
    fn operand_arity(&self) -> usize {
        3
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfConcat(operands[0], operands[1], operands[2])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "concat_array_axis_1", &first_dim_sort, &array_sort);

        let domain_sort_ = Sort::int(&context);
        let range_sort_ = Sort::int(&context);
        let array_sort_ = Sort::array(context, &domain_sort_, &range_sort_);

        let first_dim_sort_ = Sort::int(&context);
        let mut array_ = Array::fresh_const(&context,  "concat_array_axis_0", &first_dim_sort_, &array_sort_);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let axis = operands[2].vecs.select(&const0).as_array().unwrap().select(&const0).as_int().unwrap();
        let result_size_x_1 = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y_1_1 = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let result_size_y_2_1 = in2_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let result_size_y_1 = Int::add(context, &[&result_size_y_1_1, &result_size_y_2_1]);
        let result_size_y_0 = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let result_size_x_1_0 = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_x_2_0 = in2_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_x_0 = Int::add(context, &[&result_size_x_1_0, &result_size_x_2_0]);
        let result_size_x = axis._eq(&const0).ite(&result_size_x_0, &result_size_x_1);
        let result_size_y = axis._eq(&const0).ite(&result_size_y_0, &result_size_y_1);

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "concat_array_second_axis_1:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x_1);
                let in1_is_in_col = Int::lt(&col_index, &result_size_y_1_1);
                let in2_is_in_col = Int::lt(&col_index, &result_size_y_1);
                // result_size_y_1是第一个输入的纵坐标，result_size_y_1到result_size_y是第二个输入的纵坐标
                // 如果在第一个输入纵坐标内就用第一个数，如果不在第一个输入纵坐标内但是在第二个输入纵坐标内就用第二个数，否则就不动
                let value = is_in_row.ite(&in1_is_in_col.ite(&in1_value_i_j, &in2_is_in_col.ite(&in2_value_i_j, &const0)), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "concat_array_second_axis_0_1:", &domain_sort, &range_sort);

            let domain_sort_ = Sort::int(&context);
            let range_sort_ = Sort::int(&context);
            let mut array_val_ = Array::fresh_const(context, "concat_array_second_axis_0_2:", &domain_sort_, &range_sort_);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x_1_0);
                let is_in_row_ = Int::lt(&row_index, &result_size_x_2_0);
                let is_in_col = Int::lt(&col_index, &result_size_y_0);
                // 按照原来的顺序取出来，然后第二个放进去的时候横坐标偏移量是第一个数组的第一维度
                let value = is_in_row.ite(&is_in_col.ite(&in1_value_i_j, &const0), &const0);
                let value_ = is_in_row_.ite(&is_in_col.ite(&in2_value_i_j, &const0), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
                array_val_ = array_val_.store(&Int::from_i64(context, j as i64), &value_);
            }

            array_ = array_.store(&Int::from_i64(context, i as i64), &array_val);
            array_ = array_.store(&Int::add(&context, &[&Int::from_i64(context, i as i64), &result_size_x_1_0]), &array_val_);
        }

        array = axis._eq(&const0).ite(&array_, &array);

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_concat() -> Box<dyn Component> {
    Box::new(TfConcat) as _
}

#[derive(Debug)]
struct TfClipByValue;

impl Component for TfClipByValue {
    fn operand_arity(&self) -> usize {
        3
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfClipByValue(operands[0], operands[1], operands[2])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        _bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "clip_by_value_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let _in3_size =  operands[2].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        // 假定最大值最小值就在数组的第一个位置上
        let min_array = operands[1].vecs.select(&Int::from_i64(context, 0 as i64)).as_array().unwrap();
        let min = min_array.select(&Int::from_i64(context, 0 as i64)).as_int().unwrap();
        let max_array = operands[2].vecs.select(&Int::from_i64(context, 0 as i64)).as_array().unwrap();
        let max = max_array.select(&Int::from_i64(context, 0 as i64)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "clip_by_value_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let is_max = Int::gt(&in1_value_i_j, &max);
                let is_min = Int::lt(&in1_value_i_j, &min);
                let value = is_max.ite(&max, &is_min.ite(&min, &in1_value_i_j));
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_clip_by_value() -> Box<dyn Component> {
    Box::new(TfClipByValue) as _
}

#[derive(Debug)]
struct TfEqual;

impl Component for TfEqual {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "equal_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "equal_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::_eq(&in1_value_i_j, &in2_value_i_j).ite(&const1, &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
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
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfExpandDims(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        // 由于我们是二维数组，那么axis只有0、1、-1三个选项，而-1和1功能是一样的，所以统一为0和1
        let axis_array = operands[1].vecs.select(&const0).as_array().unwrap();
        let axis_ = axis_array.select(&const0).as_int().unwrap();
        let axis = axis_._eq(&const0).ite(&const0, &const1);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "expand_dims_array_axis_0", &first_dim_sort, &array_sort);

        let domain_sort_1 = Sort::int(&context);
        let range_sort_1 = Sort::int(&context);
        let array_sort_1 = Sort::array(context, &domain_sort_1, &range_sort_1);

        let first_dim_sort_1 = Sort::int(&context);
        let mut array_1 = Array::fresh_const(&context,  "expand_dims_array_axis_1", &first_dim_sort_1, &array_sort_1);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let in1_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let _in1_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        // axis = 0时，维度数组例如[2] -> [1, 2]，axis = 1时，维度数组例如[2] -> [2, 1]
        let result_size_x = axis._eq(&const0).ite(&const1, &in1_size_x);
        let result_size_y = axis._eq(&const0).ite(&in1_size_x, &const1);
        // 只能是按照两种情况求出两种数组，然后在根据axis的值确定是哪个数组了
        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "expand_dims_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                array_val = array_val.store(&Int::from_i64(context, j as i64), &in1_value_i_j);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        for i in 0 .. DIMSIZE[1] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "expand_dims_array_second_1:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[0] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                array_val = array_val.store(&Int::from_i64(context, j as i64), &in1_value_i_j);
            }

            array_1 = array_1.store(&Int::from_i64(context, i as i64), &array_val);
        }

        array = axis._eq(&const0).ite(&array, &array_1);

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_expand_dims() -> Box<dyn Component> {
    Box::new(TfExpandDims) as _
}

#[derive(Debug)]
struct TfEye;

impl Component for TfEye {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfEye(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "eye_array", &first_dim_sort, &array_sort);

        let row_array = operands[0].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let result_size_x = row_array.select(&Int::from_i64(context, 0)).as_int().unwrap();
        let col_array = operands[1].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let result_size_y = col_array.select(&Int::from_i64(context, 0)).as_int().unwrap();


        for i in 0 .. DIMSIZE[0] {
            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "eye_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                let is_row_equal_col = Int::_eq(&row_index, &col_index);
                let value = is_in_row.ite(&is_in_col.ite(&is_row_equal_col.ite(&const1, &const0), &const0), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_eye() -> Box<dyn Component> {
    Box::new(TfEye) as _
}

#[derive(Debug)]
struct TfOnes;

impl Component for TfOnes {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfOnes(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "one_array", &first_dim_sort, &array_sort);

        // 这里输入的参数就是数组的维度，如【2， 3】就是两行三列的数组
        let row_array = operands[0].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let result_size_x = row_array.select(&Int::from_i64(context, 0)).as_int().unwrap();
        let col_array = operands[0].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let result_size_y = col_array.select(&Int::from_i64(context, 1)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {
            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "one_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                let value = is_in_row.ite(&is_in_col.ite(&const1, &const0), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
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

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfZeros(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "one_array", &first_dim_sort, &array_sort);

        // 这里输入的参数就是数组的维度，如【2， 3】就是两行三列的数组
        let row_array = operands[0].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let result_size_x = row_array.select(&Int::from_i64(context, 0)).as_int().unwrap();
        let col_array = operands[0].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let result_size_y = col_array.select(&Int::from_i64(context, 1)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {
            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "one_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                // 好吧，谁也不知道这里的判断有啥用。。。反正都是0，但也不敢乱该，怕以后改掉了咋办
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                let value = is_in_row.ite(&is_in_col.ite(&const0, &const0), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_zeros() -> Box<dyn Component> {
    Box::new(TfZeros) as _
}

#[derive(Debug)]
struct TfOnesLike;

impl Component for TfOnesLike {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfOnesLike(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "ones_like_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {
            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "ones_like_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                let value = is_in_row.ite(&is_in_col.ite(&const1, &const0), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_ones_like() -> Box<dyn Component> {
    Box::new(TfOnesLike) as _
}

#[derive(Debug)]
struct TfZerosLike;

impl Component for TfZerosLike {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfZerosLike(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "zeros_like_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {
            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "zeros_like_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                let value = is_in_row.ite(&is_in_col.ite(&const0, &const0), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_zeros_like() -> Box<dyn Component> {
    Box::new(TfZerosLike) as _
}

#[derive(Debug)]
struct TfFill;

impl Component for TfFill {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfFill(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "fill_array", &first_dim_sort, &array_sort);

        // 这里输入的参数就是数组的维度，如【2， 3】就是两行三列的数组
        let row_array = operands[0].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let result_size_x = row_array.select(&Int::from_i64(context, 0)).as_int().unwrap();
        let col_array = operands[0].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let result_size_y = col_array.select(&Int::from_i64(context, 1)).as_int().unwrap();
        let fill_value_array = operands[1].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let fill_value = fill_value_array.select(&Int::from_i64(context, 0)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {
            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "fill_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                let value = is_in_row.ite(&is_in_col.ite(&fill_value, &const0), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_fill() -> Box<dyn Component> {
    Box::new(TfFill) as _
}

#[derive(Debug)]
struct TfGreater;

impl Component for TfGreater {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfGreater(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "greater_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "greater_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::gt(&in1_value_i_j, &in2_value_i_j).ite(&const1, &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_greater() -> Box<dyn Component> {
    Box::new(TfGreater) as _
}

#[derive(Debug)]
struct TfGreaterEqual;

impl Component for TfGreaterEqual {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfGreaterEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "greater_equal_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "greater_equal_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::ge(&in1_value_i_j, &in2_value_i_j).ite(&const1, &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_greater_equal() -> Box<dyn Component> {
    Box::new(TfGreaterEqual) as _
}

#[derive(Debug)]
struct TfNotEqual;

impl Component for TfNotEqual {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfNotEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "not_equal_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "not_equal_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::_eq(&in1_value_i_j, &in2_value_i_j).ite(&const0, &const1);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_not_equal() -> Box<dyn Component> {
    Box::new(TfNotEqual) as _
}

#[derive(Debug)]
struct TfNegative;

impl Component for TfNegative {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfNegative(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "negative_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "negative_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::sub(&context, &[&const0, &in1_value_i_j]);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_negative() -> Box<dyn Component> {
    Box::new(TfNegative) as _
}

#[derive(Debug)]
struct TfReciprocal;

impl Component for TfReciprocal {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfReciprocal(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const1 = one(context, bit_width);
        let const0 = zero(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "reciprocal_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "reciprocal_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                // 要注意分母不能为0
                let div_value = Int::_eq(&in1_value_i_j, &const0).ite(&const1, &in1_value_i_j);
                let value = Int::div(&const1, &div_value);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_reciprocal() -> Box<dyn Component> {
    Box::new(TfReciprocal) as _
}

#[derive(Debug)]
struct TfBincount;

impl Component for TfBincount {
    fn operand_arity(&self) -> usize {
        4
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfBincount(operands[0], operands[1], operands[2], operands[3])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "bincount_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        // 确保第二维度在min和max之间
        let min_length_array = operands[2].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let min_length = min_length_array.select(&Int::from_i64(context, 0)).as_int().unwrap();
        let max_length_array = operands[3].vecs.select(&Int::from_i64(context, 0)).as_array().unwrap();
        let max_length = max_length_array.select(&Int::from_i64(context, 0)).as_int().unwrap();
        // 记录数组长度的最大值
        let mut max_value = zero(context, bit_width);
        let zhanweifu = Int::from_i64(context, 114514);

        for i in 0 .. DIMSIZE[0] {
            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "bincount_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                max_value = Int::gt(&in1_value_i_j, &max_value).ite(&in1_value_i_j, &max_value);
                let value = Int::mul(&context, &[&in1_value_i_j, &in2_value_i_j]);
                // 确保第二维度在min和max之间
                let is_min = Int::lt(&in1_value_i_j, &min_length);
                let is_max = Int::gt(&in1_value_i_j, &max_length);
                let is_in_range = Int::lt(&in1_value_i_j, &max_value);
                // 由于之前填充了0，所以不能直接让index为0，这个时候就要判断如果不在范围之内那么就得让他在占位符里面呆着
                let index = is_min.ite(&min_length, &is_max.ite(&max_length, &is_in_range.ite(&in1_value_i_j, &zhanweifu)));
                array_val = array_val.store(&index, &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }
        let is_min = Int::lt(&max_value, &min_length);
        let is_max = Int::gt(&max_value, &max_length);
        let result_size_y = is_min.ite(&min_length, &is_max.ite(&max_length, &max_value));

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_bincount() -> Box<dyn Component> {
    Box::new(TfBincount) as _
}

#[derive(Debug)]
struct TfCountNonzero;

impl Component for TfCountNonzero {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfCountNonzero(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const1 = one(context, bit_width);
        let const0 = zero(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "count_nonzero_array", &first_dim_sort, &array_sort);
        let result_size_x = Int::from_i64(&context, 1);
        let result_size_y = Int::from_i64(&context, 1);
        let mut count = zero(context, bit_width);

        let domain_sort_1 = Sort::int(&context);
        let range_sort_1 = Sort::int(&context);
        let mut array_val = Array::fresh_const(context, "count_nonzero_array_second:", &domain_sort_1, &range_sort_1);

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let is_zero = Int::_eq(&in1_value_i_j, &const0);
                count = is_zero.ite(&count, &Int::add(&context, &[&count, &const1]));
            }
        }
        array_val = array_val.store(&Int::from_i64(context, 0), &count);
        array = array.store(&Int::from_i64(context, 0), &array_val);

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_count_nonzero() -> Box<dyn Component> {
    Box::new(TfCountNonzero) as _
}

#[derive(Debug)]
struct TfCumsum;

impl Component for TfCumsum {
    fn operand_arity(&self) -> usize {
        4
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfCumsum(operands[0], operands[1], operands[2], operands[3])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "cumsum_array_axis_0", &first_dim_sort, &array_sort);

        let domain_sort_1 = Sort::int(&context);
        let range_sort_1 = Sort::int(&context);
        let array_sort_1 = Sort::array(context, &domain_sort_1, &range_sort_1);

        let first_dim_sort_1 = Sort::int(&context);
        let mut array_1 = Array::fresh_const(&context,  "cumsum_array_axis_1", &first_dim_sort_1, &array_sort_1);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let axis_array = operands[1].vecs.select(&const0).as_array().unwrap();
        let axis = axis_array.select(&const0).as_int().unwrap();
        // 二维数组的axis只有可能是0或1
        let axis_is_zero = axis._eq(&const0);
        let exclusive_array = operands[2].vecs.select(&const0).as_array().unwrap();
        let exclusive = exclusive_array.select(&const0).as_int().unwrap();
        let exclusive_is_zero = exclusive._eq(&const0);
        let reverse_array = operands[3].vecs.select(&const0).as_array().unwrap();
        let reverse = reverse_array.select(&const0).as_int().unwrap();
        let reverse_is_zero = reverse._eq(&const0);

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "cumsum_array_second_1:", &domain_sort, &range_sort);
            // 这里防止exclusive的时候下标为0的位置没有值，如果为false也会被新的值覆盖，不会有影响
            array_val.store(&const0, &const0);
            let mut ans = zero(&context, bit_width);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                ans = Int::add(&context, &[&ans, &in1_value_i_j]);
                let value = is_in_row.ite(&is_in_col.ite(&ans, &const0), &const0);
                // exclusive可以看作整个数组向右平移一位
                // reverse可以看成下标为总长度减去当前长度
                let index_with_exclusive = exclusive_is_zero.ite(&Int::from_i64(context, j as i64), &Int::from_i64(context, (j + 1) as i64));
                let index_with_exclusive_and_reverse = reverse_is_zero.ite(&index_with_exclusive, &Int::sub(&context, &[&result_size_y, &index_with_exclusive, &const1]));
                array_val = array_val.store(&index_with_exclusive_and_reverse, &value);
            }

            array_1 = array_1.store(&Int::from_i64(context, i as i64), &array_val);
        }

        for i in 0 .. DIMSIZE[1] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "cumsum_array_second_0:", &domain_sort, &range_sort);
            // 这里防止exclusive的时候下标为0的位置没有值，如果为false也会被新的值覆盖，不会有影响
            array_val.store(&const0, &const0);
            let mut ans = zero(&context, bit_width);

            for j in 0 .. DIMSIZE[0] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                ans = Int::add(&context, &[&ans, &in1_value_i_j]);
                let value = is_in_row.ite(&is_in_col.ite(&ans, &const0), &const0);
                // exclusive可以看作整个数组向右平移一位
                // reverse可以看成下标为总长度减去当前长度
                let index_with_exclusive = exclusive_is_zero.ite(&Int::from_i64(context, j as i64), &Int::from_i64(context, (j + 1) as i64));
                let index_with_exclusive_and_reverse = reverse_is_zero.ite(&index_with_exclusive, &Int::sub(&context, &[&result_size_y, &index_with_exclusive, &const1]));
                array_val = array_val.store(&index_with_exclusive_and_reverse, &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        array = axis_is_zero.ite(&array, &array_1);

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_cumsum() -> Box<dyn Component> {
    Box::new(TfCumsum) as _
}

#[derive(Debug)]
struct TfMaximum;

impl Component for TfMaximum {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfMaximum(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        _bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "maximum_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "maximum_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::gt(&in1_value_i_j, &in2_value_i_j).ite(&in1_value_i_j, &in2_value_i_j);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
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

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfMinimum(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        _bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "minimum_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let _in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "minimum_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::lt(&in1_value_i_j, &in2_value_i_j).ite(&in1_value_i_j, &in2_value_i_j);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_minimum() -> Box<dyn Component> {
    Box::new(TfMinimum) as _
}

#[derive(Debug)]
struct TfReverse;

impl Component for TfReverse {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfReverse(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        _bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "reverse_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "reverse_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &result_size_x);
                let is_in_col = Int::lt(&col_index, &result_size_y);
                // 相反就是纵坐标总长度减去当前纵坐标，然后放到数组中
                // 要注意有一些是0，所以把这些排除掉
                let reverse_index = Int::sub(context, &[&result_size_y, &col_index]);
                let value_index = is_in_row.ite(&is_in_col.ite(&reverse_index, &col_index), &col_index);
                array_val = array_val.store(&value_index, &in1_value_i_j);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_reverse() -> Box<dyn Component> {
    Box::new(TfReverse) as _
}

#[derive(Debug)]
struct TfSign;

impl Component for TfSign {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfSquare(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);
        let const_mins_1 = Int::from_i64(context, -1);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "sign_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "sign_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                // 和0比小的返回-1，和0比大的返回1，和0相等返回0
                let is_plus = Int::gt(&in1_value_i_j, &const0);
                let is_minus = Int::lt(&in1_value_i_j, &const0);
                let value = is_minus.ite(&const_mins_1, &is_plus.ite(&const1, &const0));
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_sign() -> Box<dyn Component> {
    Box::new(TfSign) as _
}

#[derive(Debug)]
struct TfSquare;

impl Component for TfSquare {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfSquare(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        _bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "square_array", &first_dim_sort, &array_sort);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let result_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();

        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "square_array_second:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let value = Int::mul(&context, &[&in1_value_i_j, &in1_value_i_j]);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array = array.store(&Int::from_i64(context, i as i64), &array_val);
        }

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_square() -> Box<dyn Component> {
    Box::new(TfSquare) as _
}

#[derive(Debug)]
struct TfWhere;

impl Component for TfWhere {
    fn operand_arity(&self) -> usize {
        3
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        Operator::TfWhere(operands[0], operands[1], operands[2])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        let const0 = zero(context, bit_width);
        let const1 = one(context, bit_width);

        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let array_sort = Sort::array(context, &domain_sort, &range_sort);

        let first_dim_sort = Sort::int(&context);
        let mut array = Array::fresh_const(&context,  "where_array_without_2_3", &first_dim_sort, &array_sort);

        let domain_sort_1 = Sort::int(&context);
        let range_sort_1 = Sort::int(&context);
        let array_sort_1 = Sort::array(context, &domain_sort_1, &range_sort_1);

        let first_dim_sort_1 = Sort::int(&context);
        let mut array_1 = Array::fresh_const(&context,  "where_array_with_2_3", &first_dim_sort_1, &array_sort_1);

        let domain_sort_1_ = Sort::int(&context);
        let range_sort_1_ = Sort::int(&context);
        let mut array_val_1_ = Array::fresh_const(context, "where_array_second_without_2_3:", &domain_sort_1_, &range_sort_1_);

        let size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let in1_size =  operands[0].vecs.select(&size_index).as_array().unwrap();
        let in2_size =  operands[1].vecs.select(&size_index).as_array().unwrap();
        let in3_size =  operands[2].vecs.select(&size_index).as_array().unwrap();
        let in1_size_x = in1_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let in1_size_y = in1_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let in2_size_x = in2_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let in2_size_y = in2_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let in3_size_x = in3_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let in3_size_y = in3_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        // 记录下第一种情况中行的个数，由于行的个数由非0的个数来计算，因此需要一个变量
        let mut count_result_size_x = zero(context, bit_width);
        // 和boolean_mask一样，需要一个变量放下多余的变量用来处理
        let zhanweifu = Int::from_i64(&context, 114514);
        // 由于第二个和第三个输入都是0和不都是0是不一样的功能，因此得先处理是不是都是0
        let mut is_all_zero = Bool::from_bool(&context, true);

        for i in 0 .. DIMSIZE[0] {
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in3_value = operands[2].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            for j in 0 .. DIMSIZE[1] {
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in3_value_i_j = in3_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in2_in_row = Int::lt(&row_index, &in2_size_x);
                let is_in2_in_col = Int::lt(&col_index, &in2_size_y);
                let is_in3_in_row = Int::lt(&row_index, &in3_size_x);
                let is_in3_in_col = Int::lt(&col_index, &in3_size_y);
                let in2_is_zero = in2_value_i_j._eq(&const0);
                let in3_is_zero = in3_value_i_j._eq(&const0);
                let false_value = Bool::from_bool(&context, false);
                is_all_zero = is_in2_in_row.ite(
                    &is_in2_in_col.ite(
                        &is_in3_in_row.ite(
                            &is_in3_in_col.ite(
                                &in2_is_zero.ite(
                                    &in3_is_zero, 
                                    &false_value), 
                                    &is_all_zero), 
                                    &is_all_zero), 
                                    &is_all_zero), 
                                    &is_all_zero);
            }
        }
        // 如果第二个和第三个都没输入（也就是都是0）那就统计非0的下标，一维数组就只有行，二维数组有行有列
        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &in1_size_x);
                let is_in_col = Int::lt(&col_index, &in1_size_y);
                let false_value = Bool::from_bool(&context, false);
                let is_zero = is_in_row.ite(&is_in_col.ite(&in1_value_i_j._eq(&const0), &false_value), &false_value);
                // 一维数组只存一个维度，也就是纵坐标，二维数组要存两个维度，第一个是纵坐标，第二个是横坐标
                let one_dim_array = array_val_1_.store(&is_zero.ite(&zhanweifu, &const0), &col_index);
                let two_dim_array = array_val_1_.store(&is_zero.ite(&zhanweifu, &const0), &row_index)
                                                        .store(&is_zero.ite(&zhanweifu, &const1), &col_index);
                count_result_size_x = is_zero.ite(&count_result_size_x, &Int::add(&context, &[&count_result_size_x, &const1]));
                array_val_1_ = in1_size_x._eq(&const1).ite(&one_dim_array, &two_dim_array);
            }
        }

        array = array.store(&const0, &array_val_1_);
        //否则就比较，如果第一个数组的位置的值为真则返回第二的数组位置的值，否则是第三个数组位置的值
        for i in 0 .. DIMSIZE[0] {

            let in1_value = operands[0].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in2_value = operands[1].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();
            let in3_value = operands[2].vecs.select(&Int::from_i64(context, i as i64)).as_array().unwrap();

            let domain_sort = Sort::int(&context);
            let range_sort = Sort::int(&context);
            let mut array_val = Array::fresh_const(context, "where_array_second_with_2_3:", &domain_sort, &range_sort);

            for j in 0 .. DIMSIZE[1] {
                let in1_value_i_j = in1_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in2_value_i_j = in2_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let in3_value_i_j = in3_value.select(&Int::from_i64(context, j as i64)).as_int().unwrap();
                let row_index = Int::from_i64(context, i as i64);
                let col_index = Int::from_i64(context, j as i64);
                let is_in_row = Int::lt(&row_index, &in2_size_x);
                let is_in_col = Int::lt(&col_index, &in2_size_y);
                let value = is_in_row.ite(&is_in_col.ite(&in1_value_i_j._eq(&const0).ite(&in2_value_i_j, &in3_value_i_j), &const0), &const0);
                array_val = array_val.store(&Int::from_i64(context, j as i64), &value);
            }

            array_1 = array_1.store(&Int::from_i64(context, i as i64), &array_val);
        }

        array = is_all_zero.ite(&array, &array_1);
        // 最终长度，第一种情况第一维度就是非0的个数，第二维度就是第一个输入的行数
        // 第二种情况形状不变
        let result_size_x_without_2_3 = count_result_size_x.clone();
        let result_size_y_without_2_3 = in1_size_x.clone();
        let result_size_x_with_2_3 = in2_size.select(&Int::from_i64(&context, SIZE_X)).as_int().unwrap();
        let result_size_y_with_2_3 = in2_size.select(&Int::from_i64(&context, SIZE_Y)).as_int().unwrap();
        let result_size_x = is_all_zero.ite(&result_size_x_without_2_3, &result_size_x_with_2_3);
        let result_size_y = is_all_zero.ite(&result_size_y_without_2_3, &result_size_y_with_2_3);

        let array_size_index = Int::from_i64(&context, SIZE_STORE_INDEX);
        let domain_sort = Sort::int(&context);
        let range_sort = Sort::int(&context);
        let mut array_val = Array::fresh_const(&context, "array_size:", &domain_sort, &range_sort);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_X), &result_size_x);
        array_val = array_val.store(&Int::from_i64(&context, SIZE_Y), &result_size_y);
        array = array.store(&array_size_index, &array_val);

        let result = Vecs::new(operands[0].dims, array);
        return result;
    }
}

pub fn tf_where() -> Box<dyn Component> {
    Box::new(TfWhere) as _
}

macro_rules! with_operator_component {
    ( $me:expr , |$c:ident| $body:expr ) => {
        match $me {
            Operator::Var => panic!("`Var` operators do not have a component"),
            //Operator::Vecs(_) =>panic!("`Vecs` operators do not have a component"),
            /*Operator::Vecs(_) => {
                let $c = Vecs;
                $body
            }*/
            // Operator::Const(c) => {
            //     let $c = Const(c.to_vec());
            //     $body
            // }
            Operator::TfAbs(_) => {
                let $c = TfAbs;
                $body
            }
            Operator::TfAdd(_, _) => {
                let $c = TfAdd;
                $body
            }
            Operator::TfMul(_, _) => {
                let $c = TfMul;
                $body
            }
            Operator::TfDiv(_, _) => {
                let $c = TfDiv;
                $body
            }
            Operator::TfArgMax(_, _) => {
                let $c = TfArgMax;
                $body
            }
            Operator::TfArgMin(_, _) => {
                let $c = TfArgMin;
                $body
            }
            Operator::TfBooleanMask(_, _) => {
                let $c = TfBooleanMask;
                $body
            }
            Operator::TfCast(_) => {
                let $c = TfCast;
                $body
            }
            Operator::TfClipByValue(_, _, _) => {
                let $c = TfClipByValue;
                $body
            }
            Operator::TfConcat(_, _, _) => {
                let $c = TfConcat;
                $body
            }
            Operator::TfEqual(_, _) => {
                let $c = TfEqual;
                $body
            }
            Operator::TfExpandDims(_, _) => {
                let $c = TfExpandDims;
                $body
            }
            Operator::TfEye(_, _) => {
                let $c = TfEye;
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
            Operator::TfOnesLike(_) => {
                let $c = TfOnesLike;
                $body
            }
            Operator::TfZerosLike(_) => {
                let $c = TfZerosLike;
                $body
            }
            Operator::TfFill(_, _) => {
                let $c = TfFill;
                $body
            }
            Operator::TfGreater(_, _) => {
                let $c = TfGreater;
                $body
            }
            Operator::TfGreaterEqual(_, _) => {
                let $c = TfGreaterEqual;
                $body
            }
            Operator::TfNotEqual(_, _) => {
                let $c = TfNotEqual;
                $body
            }
            Operator::TfNegative(_) => {
                let $c = TfNegative;
                $body
            }
            Operator::TfReciprocal(_) => {
                let $c = TfReciprocal;
                $body
            }
            Operator::TfBincount(_, _, _, _) => {
                let $c = TfBincount;
                $body
            }
            Operator::TfCountNonzero(_) => {
                let $c = TfCountNonzero;
                $body
            }
            Operator::TfCumsum(_, _, _, _) => {
                let $c = TfCumsum;
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
            Operator::TfReverse(_) => {
                let $c = TfReverse;
                $body
            }
            Operator::TfSign(_) => {
                let $c = TfSign;
                $body
            }
            Operator::TfSquare(_) => {
                let $c = TfSquare;
                $body
            }
            Operator::TfWhere(_, _, _) => {
                let $c = TfWhere;
                $body
            }
        }
    };
}

impl Component for Operator {
    fn operand_arity(&self) -> usize {
        Operator::arity(self)
    }

    fn make_operator(&self, immediates: &Vec<Vecs<Vec<Vec<i64>>>>, operands: &[Id]) -> Operator {
        with_operator_component!(self, |c| c.make_operator(immediates, operands))
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        immediates: &[Vecs<Array<'a>>],
        operands: &[Vecs<Array<'a>>],
        bit_width: u32,
    ) -> Vecs<Array<'a>> {
        with_operator_component!(self, |c| {
            c.make_expression(context, immediates, operands, bit_width)
        })
    }

    fn immediate_arity(&self) -> usize {
        with_operator_component!(self, |c| c.immediate_arity())
    }
}