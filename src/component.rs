use crate::{Id, Operator, Vecs, DIMS};
use std::{fmt::Debug, usize};
use z3::ast::{Int, Ast};

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
                result.vecs[i].push(operands[0].vecs[i][j]._eq(&operands[1].vecs[i][j]).ite(&const1, &const0));
            }
        }

        return result;
    }
}

pub fn tf_equal() -> Box<dyn Component> {
    Box::new(TfEqual) as _
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
            Operator::TfCast(_) => {
                let $c = TfCast;
                $body
            }
            Operator::TfConstant(_) => {
                let $c = TfConstant;
                $body
            }
            Operator::TfEqual(_, _) => {
                let $c = TfEqual;
                $body
            }
            Operator::TfMultiply(_, _) => {
                let $c = TfMultiply;
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