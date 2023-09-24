use crate::{Id, Operator};
use std::fmt::Debug;
use z3::ast::{Ast, BV as BitVec};

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

fn bit_vec_from_u64(context: &z3::Context, val: u64, bit_width: u32) -> BitVec {
    BitVec::from_i64(context, val as i64, bit_width)
}

fn zero(context: &z3::Context, bit_width: u32) -> BitVec {
    bit_vec_from_u64(context, 0, bit_width)
}

fn one(context: &z3::Context, bit_width: u32) -> BitVec {
    bit_vec_from_u64(context, 1, bit_width)
}

pub trait Component: Debug {
    fn operand_arity(&self) -> usize;

    fn make_operator(&self, immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator;

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        // immediates: &[BitVec<'a>],
        // operands: &[BitVec<'a>],
        immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],

        bit_width: u32,
    ) -> 
        //BitVec<'a> 
        Vec<BitVec<'a>>;
        
    

    /// How many immediates does this component require?
    fn immediate_arity(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct Const(Option<u64>);

impl Component for Const {
    fn operand_arity(&self) -> usize {
        0
    }

    fn make_operator(&self, immediates: &Vec<Vec<u64>>, _operands: &[Id]) -> Operator {
        if let Some(val) = self.0 {
            Operator::Const(val)
        } else {
            Operator::Const(immediates[0][0])
        }
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        _operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {

        // if let Some(val) = self.0 {
        //     BitVec::from_i64(context, val as i64, bit_width)
        // } else {
        //     immediates[0][0].clone()
        // }

        let mut result : Vec<BitVec<'a>> = Vec::new();

        if let Some(val) = self.0 {
            //直接设置出val个val数字
            for _i in 1 .. val + 1 {
                result.push(BitVec::from_i64(context, val as i64, bit_width));
            }
            
        } else {
            //当没有手动设置的时候，就生产10个
            for _i in 1 .. 10 + 1 {
                result.push(BitVec::from_i64(context, 10, bit_width));
            }
        }

        /*if let Some(val) = self.0 {
            result.push(BitVec::from_i64(context, val as i64, bit_width));
        } else {
            result.push(immediates[0][0].clone());
        }*/

        return result;
    }

    fn immediate_arity(&self) -> usize {
        if self.0.is_some() {
            0
        } else {
            1
        }
    }
}

pub fn const_(val: Option<u64>) -> Box<dyn Component> {
    Box::new(Const(val)) as _
}

/* 
#[derive(Debug)]
struct Vecs(Option<u64>);

impl Component for Vecs {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, immediates: &Vec<Vec<u64>>, _operands: &[Id]) -> Operator {
        Operator::Vecs(self.0.unwrap())
        
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        // immediates: &[BitVec<'a>],
        // _operands: &[BitVec<'a>],
        immediates: &[Vec<BitVec<'a>>],
        _operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> 
        //BitVec<'a> 
        Vec<BitVec<'a>>
        {
        let mut result : Vec<BitVec<'a>> = Vec::new();
        let sz = BitVec::as_u64(&_operands[0][0]).unwrap();
        
        for i in 1 .. sz  {
            if let Some(val) = self.0 {
                result.push(BitVec::from_i64(context, val as i64, bit_width));
            } else {
                result.push(immediates[0][0].clone());
            }  
        }

        return result;
    }

    fn immediate_arity(&self) -> usize {
         0
    }
}

pub fn vecs_(val: Option<u64>) -> Box<dyn Component> {
    Box::new(Vecs(val)) as _
}*/


/* 
// #[derive(Debug)]
// struct Eqz;

// impl Component for Eqz {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Eqz(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         zero(context, bit_width)
//             ._eq(&operands[0])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn eqz() -> Box<dyn Component> {
//     Box::new(Eqz) as _
// }

// #[derive(Debug)]
// struct Clz;

// impl Component for Clz {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Clz(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         fn clz<'a>(
//             context: &'a z3::Context,
//             input: &BitVec<'a>,
//             one_bit: &BitVec<'a>,
//             bit_width: u32,
//             i: u32,
//         ) -> BitVec<'a> {
//             if i == bit_width {
//                 bit_vec_from_u64(context, i as u64, bit_width)
//             } else {
//                 input
//                     .extract(bit_width - 1 - i, bit_width - 1 - i)
//                     ._eq(&one_bit)
//                     .ite(
//                         &bit_vec_from_u64(context, i as u64, bit_width),
//                         &clz(context, input, one_bit, bit_width, i + 1),
//                     )
//             }
//         }

//         let one_bit = BitVec::from_i64(context, 1, 1);
//         clz(context, &operands[0], &one_bit, bit_width, 0)
//     }
// }

// pub fn clz() -> Box<dyn Component> {
//     Box::new(Clz) as _
// }

// #[derive(Debug)]
// struct Ctz;

// impl Component for Ctz {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Ctz(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         fn ctz<'a>(
//             context: &'a z3::Context,
//             input: &BitVec<'a>,
//             one_bit: &BitVec<'a>,
//             bit_width: u32,
//             i: u32,
//         ) -> BitVec<'a> {
//             if i == bit_width {
//                 bit_vec_from_u64(context, i as u64, bit_width)
//             } else {
//                 input.extract(i, i)._eq(&one_bit).ite(
//                     &bit_vec_from_u64(context, i as u64, bit_width),
//                     &ctz(context, input, one_bit, bit_width, i + 1),
//                 )
//             }
//         }

//         let one_bit = BitVec::from_i64(context, 1, 1);
//         ctz(context, &operands[0], &one_bit, bit_width, 0)
//     }
// }

// pub fn ctz() -> Box<dyn Component> {
//     Box::new(Ctz) as _
// }

// #[derive(Debug)]
// struct Popcnt;

// impl Component for Popcnt {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Popcnt(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         let mut bits: Vec<_> = (0..bit_width)
//             .map(|i| operands[0].extract(i, i).zero_ext(bit_width - 1))
//             .collect();
//         let initial = bits.pop().unwrap();
//         bits.iter().fold(initial, |a, b| a.bvadd(b))
//     }
// }

// pub fn popcnt() -> Box<dyn Component> {
//     Box::new(Popcnt) as _
// }

// #[derive(Debug)]
// struct Eq;

// impl Component for Eq {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Eq(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             ._eq(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn eq() -> Box<dyn Component> {
//     Box::new(Eq) as _
// }

// #[derive(Debug)]
// struct Ne;

// impl Component for Ne {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Ne(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             ._eq(&operands[1])
//             .ite(&zero(context, bit_width), &one(context, bit_width))
//     }
// }

// pub fn ne() -> Box<dyn Component> {
//     Box::new(Ne) as _
// }

// #[derive(Debug)]
// struct LtS;

// impl Component for LtS {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::LtS(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             .bvslt(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn lt_s() -> Box<dyn Component> {
//     Box::new(LtS) as _
// }

// #[derive(Debug)]
// struct LtU;

// impl Component for LtU {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::LtU(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             .bvult(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn lt_u() -> Box<dyn Component> {
//     Box::new(LtU) as _
// }

// #[derive(Debug)]
// struct GtS;

// impl Component for GtS {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::GtS(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             .bvsgt(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn gt_s() -> Box<dyn Component> {
//     Box::new(GtS) as _
// }

// #[derive(Debug)]
// struct GtU;

// impl Component for GtU {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::GtU(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             .bvugt(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn gt_u() -> Box<dyn Component> {
//     Box::new(GtU) as _
// }

// #[derive(Debug)]
// struct LeS;

// impl Component for LeS {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::LeS(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             .bvsle(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn le_s() -> Box<dyn Component> {
//     Box::new(LeS) as _
// }

// #[derive(Debug)]
// struct LeU;

// impl Component for LeU {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::LeU(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             .bvule(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn le_u() -> Box<dyn Component> {
//     Box::new(LeU) as _
// }

// #[derive(Debug)]
// struct GeS;

// impl Component for GeS {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::GeS(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             .bvsge(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn ge_s() -> Box<dyn Component> {
//     Box::new(GeS) as _
// }

// #[derive(Debug)]
// struct GeU;

// impl Component for GeU {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::GeU(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             .bvuge(&operands[1])
//             .ite(&one(context, bit_width), &zero(context, bit_width))
//     }
// }

// pub fn ge_u() -> Box<dyn Component> {
//     Box::new(GeU) as _
// }

// #[derive(Debug)]
// struct Add;

// impl Component for Add {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Add(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvadd(&operands[1])
//     }
// }

// pub fn add() -> Box<dyn Component> {
//     Box::new(Add) as _
// }

// #[derive(Debug)]
// struct Sub;

// impl Component for Sub {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Sub(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvsub(&operands[1])
//     }
// }

// pub fn sub() -> Box<dyn Component> {
//     Box::new(Sub) as _
// }

// #[derive(Debug)]
// struct Mul;

// impl Component for Mul {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Mul(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvmul(&operands[1])
//     }
// }

// pub fn mul() -> Box<dyn Component> {
//     Box::new(Mul) as _
// }

// #[derive(Debug)]
// struct DivS;

// impl Component for DivS {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::DivS(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvsdiv(&operands[1])
//     }
// }

// pub fn div_s() -> Box<dyn Component> {
//     Box::new(DivS) as _
// }

// #[derive(Debug)]
// struct DivU;

// impl Component for DivU {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::DivU(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvudiv(&operands[1])
//     }
// }

// pub fn div_u() -> Box<dyn Component> {
//     Box::new(DivU) as _
// }

// #[derive(Debug)]
// struct RemS;

// impl Component for RemS {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::RemS(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvsrem(&operands[1])
//     }
// }

// pub fn rem_s() -> Box<dyn Component> {
//     Box::new(RemS) as _
// }

// #[derive(Debug)]
// struct RemU;

// impl Component for RemU {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::RemU(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvurem(&operands[1])
//     }
// }

// pub fn rem_u() -> Box<dyn Component> {
//     Box::new(RemU) as _
// }

// #[derive(Debug)]
// struct And;

// impl Component for And {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::And(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvand(&operands[1])
//     }
// }

// pub fn and() -> Box<dyn Component> {
//     Box::new(And) as _
// }

// #[derive(Debug)]
// struct Or;

// impl Component for Or {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Or(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvor(&operands[1])
//     }
// }

// pub fn or() -> Box<dyn Component> {
//     Box::new(Or) as _
// }

// #[derive(Debug)]
// struct Xor;

// impl Component for Xor {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Xor(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvxor(&operands[1])
//     }
// }

// pub fn xor() -> Box<dyn Component> {
//     Box::new(Xor) as _
// }

// #[derive(Debug)]
// struct Shl;

// impl Component for Shl {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Shl(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvshl(&operands[1])
//     }
// }

// pub fn shl() -> Box<dyn Component> {
//     Box::new(Shl) as _
// }

// #[derive(Debug)]
// struct ShrS;

// impl Component for ShrS {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::ShrS(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvashr(&operands[1])
//     }
// }

// pub fn shr_s() -> Box<dyn Component> {
//     Box::new(ShrS) as _
// }

// #[derive(Debug)]
// struct ShrU;

// impl Component for ShrU {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::ShrU(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvlshr(&operands[1])
//     }
// }

// pub fn shr_u() -> Box<dyn Component> {
//     Box::new(ShrU) as _
// }

// #[derive(Debug)]
// struct Rotl;

// impl Component for Rotl {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Rotl(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvrotl(&operands[1])
//     }
// }

// pub fn rotl() -> Box<dyn Component> {
//     Box::new(Rotl) as _
// }

// #[derive(Debug)]
// struct Rotr;

// impl Component for Rotr {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Rotr(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         _bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0].bvrotr(&operands[1])
//     }
// }

// pub fn rotr() -> Box<dyn Component> {
//     Box::new(Rotr) as _
// }

// #[derive(Debug)]
// struct Select;

// impl Component for Select {
//     fn operand_arity(&self) -> usize {
//         3
//     }

//     fn make_operator(&self, _immediates: &[u64], operands: &[Id]) -> Operator {
//         Operator::Select(operands[0], operands[1], operands[2])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[BitVec<'a>],
//         operands: &[BitVec<'a>],
//         bit_width: u32,
//     ) -> BitVec<'a> {
//         operands[0]
//             ._eq(&zero(context, bit_width))
//             .ite(&operands[2], &operands[1])
//     }
// }

// pub fn select() -> Box<dyn Component> {
//     Box::new(Select) as _
// } */

#[derive(Debug)]
struct TfAbs;

impl Component for TfAbs {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfAbs(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        //TODO：目前只是一维

        // 取operands[0]作为输入
        // 取相同长度并且填充为0的数组，作取相反数的结果用
        let const0 = zero(context, bit_width);
        let sz = operands[0].len();
        let mut result: Vec<BitVec> = Vec::new();
        // 它（for循环）是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
        // 所以这里是左闭右开区间
        for i in 0..sz {
            //计算每一个元素的相反数，作为后面的判断，如果是正数就直接返回，负数返回相反数
            let minus_num = const0.bvsub(&operands[0][i]);
            // 判断输入的数是正数还是负数
            let plus_or_minus = operands[0][i].bvslt(&const0).ite(&one(context, bit_width), &zero(context, bit_width));
            // 正数直接返回，负数返回相反数
            result.push(plus_or_minus._eq(&one(context, bit_width)).ite(&minus_num, &operands[0][i]))
        }
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfAdd(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 获取两个输入的长度
        let size0 = operands[0].len();
        let size1 = operands[1].len();
        let mut result: Vec<BitVec> = Vec::new();
        // 如果输入等长，那么数组每个元素进行相加计算
        if size0 == size1 {
            for index in 0..size0 {
                result.push(operands[0][index].bvadd(&operands[1][index]));
            }
        } else { 
            // 如果输入不等长，那么先把短的数组扩充到和长的数组一样长（填充0）然后再相加
            let const0 = zero(context, bit_width);
            if size0 < size1 {
                // 第一个输入比第二个输入短，先计算公共部分，然后第二个输入多出来的部分加上0即可
                for index in 0..size0 {
                    result.push(operands[1][index].bvadd(&operands[0][index]));
                }
                for index in 0..(size1 - size0) {
                    result.push(operands[1][index].bvadd(&const0));
                }
            } else {
                // 第二个输入比第一个输入短，先计算公共部分，然后第一个输入多出来的部分加上0即可
                for index in 0..size1 {
                    result.push(operands[0][index].bvadd(&operands[1][index]));
                }
                for index in 0..(size0 - size1) {
                    result.push(operands[0][index].bvadd(&const0));
                }
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfMul(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 获取两个输入的长度
        let size0 = operands[0].len();
        let size1 = operands[1].len();
        let mut result: Vec<BitVec> = Vec::new();
        // 如果输入等长，那么数组每个元素进行相乘计算
        if size0 == size1 {
            for index in 0..size0 {
                result.push(operands[0][index].bvmul(&operands[1][index]));
            }
        } else { 
            // 如果输入不等长，那么先把短的数组扩充到和长的数组一样长（填充1）然后再相乘
            let const1 = one(context, bit_width);
            if size0 < size1 {
                // 第一个输入比第二个输入短，先计算公共部分，然后第二个输入多出来的部分乘上1即可
                for index in 0..size0 {
                    result.push(operands[1][index].bvmul(&operands[0][index]));
                }
                for index in 0..(size1 - size0) {
                    result.push(operands[1][index].bvmul(&const1));
                }
            } else {
                // 第二个输入比第一个输入短，先计算公共部分，然后第一个输入多出来的部分乘上1即可
                for index in 0..size1 {
                    result.push(operands[0][index].bvmul(&operands[1][index]));
                }
                for index in 0..(size0 - size1) {
                    result.push(operands[0][index].bvmul(&const1));
                }
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfDiv(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 获取两个输入的长度
        let size0 = operands[0].len();
        let size1 = operands[1].len();
        let mut result: Vec<BitVec> = Vec::new();
        // 如果输入等长，那么数组每个元素进行相除计算
        if size0 == size1 {
            for index in 0..size0 {
                result.push(operands[0][index].bvsdiv(&operands[1][index]));
            }
        } else { 
            // 如果输入不等长，那么先把短的数组扩充到和长的数组一样长（填充1）然后再相除
            let const1 = one(context, bit_width);
            if size0 < size1 {
                // 第一个输入比第二个输入短，先计算公共部分，然后第二个输入多出来的部分除以1即可
                for index in 0..size0 {
                    result.push(operands[1][index].bvsdiv(&operands[0][index]));
                }
                for index in 0..(size1 - size0) {
                    result.push(operands[1][index].bvsdiv(&const1));
                }
            } else {
                // 第二个输入比第一个输入短，先计算公共部分，然后第一个输入多出来的部分除以1即可
                for index in 0..size1 {
                    result.push(operands[0][index].bvsdiv(&operands[1][index]));
                }
                for index in 0..(size0 - size1) {
                    result.push(operands[0][index].bvsdiv(&const1));
                }
            }
        }
        return result;
    }
}

pub fn tf_div() -> Box<dyn Component> {
    Box::new(TfDiv) as _
}

#[derive(Debug)]
struct TfBooleanMask;

impl Component for TfBooleanMask {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfBooleanMask(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 获取两个输入的长度，等长后进行后续操作
        let size0 = operands[0].len();
        let size1 = operands[1].len();
        let mut result: Vec<BitVec> = Vec::new();
        // 填充物，如果掩码部分不是1，那么就返回0
        let const0 = zero(context, bit_width);
        if size0 != size1 {
            // TODO: 这里可以报错，或者干别的
        } else {
            for index in 0..size0 {
                //如果掩码为1，则返回自身，如果掩码为0，则返回0
                result.push(operands[1][index]._eq(&one(context, bit_width)).ite(&operands[0][index], &const0));
            }
        }
        return result;
    }
}

pub fn tf_boolean_mask() -> Box<dyn Component> {
    Box::new(TfBooleanMask) as _
}

#[derive(Debug)]
struct TfClipByValue;

impl Component for TfClipByValue {
    fn operand_arity(&self) -> usize {
        3
    }

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfClipByValue(operands[0], operands[1], operands[2])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 第一个是输入的数组，第二个是最小值，第三个是最大值
        // 要求数组里每一项都要和最大值和最小值比较，在此范围（含最大值和最小值）之外的，小的换成最小值，大的换成最大值
        let size = operands[0].len();
        // 目前仅有var数组的情况，还不清楚会不会单独搞一个var变量，先用数组的方法取值即可
        let min_value = &operands[1][0];
        let max_value = &operands[2][0];
        let mut result: Vec<BitVec> = Vec::new();
        for index in 0..size {
            // 判断当前值是否小于等于最小值，当前值是否大于等于最大值，1则为成立，0则为不成立
            let is_min = operands[0][index].bvsle(&min_value).ite(&one(context, bit_width), &zero(context, bit_width));
            let is_max = operands[0][index].bvsge(&max_value).ite(&one(context, bit_width), &zero(context, bit_width));
            // 先判断是不是比最小值小，如果是则取最小值，再比较是不是比最大值大，如果是则取最大值，其余情况原值返回
            result.push(operands[0][index]._eq(&is_min).ite(&min_value, &operands[0][index]._eq(&is_max).ite(&max_value, &operands[0][index])));
        }
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 依次比较两个数组每个元素是否相等即可
        // TODO：名义上如果不等长要考虑广播，下一步需要考虑不等长的情况
        let size0 = operands[0].len();
        let _size1 = operands[1].len();
        let mut result: Vec<BitVec> = Vec::new();
        for index in 0..size0 {
            result.push(operands[0][index]._eq(&operands[1][index]).ite(&one(context, bit_width), &zero(context, bit_width)));
        }
        return result;
    }
}

pub fn tf_equal() -> Box<dyn Component> {
    Box::new(TfEqual) as _
}

#[derive(Debug)]
struct TfFill;

impl Component for TfFill {
    fn operand_arity(&self) -> usize {
        2
    }

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfFill(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        _bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 根据第一个输入的长度填充第二个数
        // 不过吧。。。具体实现的话，既然数组长度已经通过其他途径确定了，这第一个量有何用。。。
        // 而且吧。。。咱所有的输入都是数组，那第二个输入可以看成一个数组，直接返回不就好了。。。
        return operands[1].to_vec();
        /* 好吧，这一段应该是想象中的填充，只不过第一个变量里面的长度不一定是最终的长度
        let length = &operands[0][0];
        let mut result: Vec<BitVec> = Vec::new();
        for index in 0..length {
            result.push(operands[1][index])
        }
        return result;
        */
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfGreater(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 依次比较两个数组每个元素是否大于即可
        // TODO：名义上如果不等长要考虑广播，下一步需要考虑不等长的情况
        let size0 = operands[0].len();
        let _size1 = operands[1].len();
        let mut result: Vec<BitVec> = Vec::new();
        for index in 0..size0 {
            result.push(operands[0][index].bvsgt(&operands[1][index]).ite(&one(context, bit_width), &zero(context, bit_width)));
        }
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfGreaterEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 依次比较两个数组每个元素是否大于等于即可
        // TODO：名义上如果不等长要考虑广播，下一步需要考虑不等长的情况
        let size0 = operands[0].len();
        let _size1 = operands[1].len();
        let mut result: Vec<BitVec> = Vec::new();
        for index in 0..size0 {
            result.push(operands[0][index].bvsge(&operands[1][index]).ite(&one(context, bit_width), &zero(context, bit_width)));
        }
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfNotEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 依次比较两个数组每个元素是否大于不等于即可
        // TODO：名义上如果不等长要考虑广播，下一步需要考虑不等长的情况
        let size0 = operands[0].len();
        let _size1 = operands[1].len();
        let mut result: Vec<BitVec> = Vec::new();
        for index in 0..size0 {
            result.push(operands[0][index].bvsge(&operands[1][index]).ite(&zero(context, bit_width), &one(context, bit_width)));
        }
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfNegative(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 依次遍历每个数，取相反数即可
        // 相反数可以采用常数零减去当前数的方法
        let const0 = zero(context, bit_width);
        let size = operands[0].len();
        let mut result: Vec<BitVec> = Vec::new();
        for index in 0..size {
            result.push(const0.bvsub(&operands[0][index]));
        }
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

    fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        Operator::TfReciprocal(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        // 依次遍历每个数，取倒数即可
        // 倒数可以采用常数1除以当前数的方法
        let const1 = one(context, bit_width);
        let size = operands[0].len();
        let mut result: Vec<BitVec> = Vec::new();
        for index in 0..size {
            result.push(const1.bvsdiv(&operands[0][index]));
        }
        return result;
    }
}

pub fn tf_reciprocal() -> Box<dyn Component> {
    Box::new(TfReciprocal) as _
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
            Operator::Const(c) => {
                let $c = Const(Some(*c));
                $body
            }
            // Operator::Eqz(_) => {
            //     let $c = Eqz;
            //     $body
            // }
            // Operator::Clz(_) => {
            //     let $c = Clz;
            //     $body
            // }
            // Operator::Ctz(_) => {
            //     let $c = Ctz;
            //     $body
            // }
            // Operator::Popcnt(_) => {
            //     let $c = Popcnt;
            //     $body
            // }
            // Operator::Eq(_, _) => {
            //     let $c = Eq;
            //     $body
            // }
            // Operator::Ne(_, _) => {
            //     let $c = Ne;
            //     $body
            // }
            // Operator::LtS(_, _) => {
            //     let $c = LtS;
            //     $body
            // }
            // Operator::LtU(_, _) => {
            //     let $c = LtU;
            //     $body
            // }
            // Operator::GtS(_, _) => {
            //     let $c = GtS;
            //     $body
            // }
            // Operator::GtU(_, _) => {
            //     let $c = GtU;
            //     $body
            // }
            // Operator::LeS(_, _) => {
            //     let $c = LeS;
            //     $body
            // }
            // Operator::LeU(_, _) => {
            //     let $c = LeU;
            //     $body
            // }
            // Operator::GeS(_, _) => {
            //     let $c = GeS;
            //     $body
            // }
            // Operator::GeU(_, _) => {
            //     let $c = GeU;
            //     $body
            // }
            // Operator::Add(_, _) => {
            //     let $c = Add;
            //     $body
            // }
            // Operator::Sub(_, _) => {
            //     let $c = Sub;
            //     $body
            // }
            // Operator::Mul(_, _) => {
            //     let $c = Mul;
            //     $body
            // }
            // Operator::DivS(_, _) => {
            //     let $c = DivS;
            //     $body
            // }
            // Operator::DivU(_, _) => {
            //     let $c = DivU;
            //     $body
            // }
            // Operator::RemS(_, _) => {
            //     let $c = RemS;
            //     $body
            // }
            // Operator::RemU(_, _) => {
            //     let $c = RemU;
            //     $body
            // }
            // Operator::And(_, _) => {
            //     let $c = And;
            //     $body
            // }
            // Operator::Or(_, _) => {
            //     let $c = Or;
            //     $body
            // }
            // Operator::Xor(_, _) => {
            //     let $c = Xor;
            //     $body
            // }
            // Operator::Shl(_, _) => {
            //     let $c = Shl;
            //     $body
            // }
            // Operator::ShrS(_, _) => {
            //     let $c = ShrS;
            //     $body
            // }
            // Operator::ShrU(_, _) => {
            //     let $c = ShrU;
            //     $body
            // }
            // Operator::Rotl(_, _) => {
            //     let $c = Rotl;
            //     $body
            // }
            // Operator::Rotr(_, _) => {
            //     let $c = Rotr;
            //     $body
            // }
            // Operator::Select(_, _, _) => {
            //     let $c = Select;
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
            Operator::TfBooleanMask(_, _) => {
                let $c = TfBooleanMask;
                $body
            }
            Operator::TfClipByValue(_, _, _) => {
                let $c = TfClipByValue;
                $body
            }
            Operator::TfEqual(_, _) => {
                let $c = TfEqual;
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
        }
    };
}

impl Component for Operator {
    fn operand_arity(&self) -> usize {
        Operator::arity(self)
    }

    fn make_operator(&self, immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
        with_operator_component!(self, |c| c.make_operator(immediates, operands))
    }

    // fn make_expression<'a>(
    //     &self,
    //     context: &'a z3::Context,
    //     immediates: &[BitVec<'a>],
    //     operands: &[BitVec<'a>],
    //     bit_width: u32,
    // ) -> BitVec<'a> {
    //     with_operator_component!(self, |c| {
    //         c.make_expression(context, immediates, operands, bit_width)
    //     })
    // }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        immediates: &[Vec<BitVec<'a>>],
        operands: &[Vec<BitVec<'a>>],
        bit_width: u32,
    ) -> Vec<BitVec<'a>> {
        with_operator_component!(self, |c| {
            c.make_expression(context, immediates, operands, bit_width)
        })
    }

    fn immediate_arity(&self) -> usize {
        with_operator_component!(self, |c| c.immediate_arity())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ctz_test() {
//         let _ = env_logger::try_init();
//         let cfg = z3::Config::new();
//         let ctx = z3::Context::new(&cfg);

//         // 0000 0000 0000 0000 0000 0000 0000 0010
//         assert!(ctz()
//             .make_expression(&ctx, &vec![], &vec![bit_vec_from_u64(&ctx, 2, 32)], 32)
//             ._eq(&bit_vec_from_u64(&ctx, 1, 32))
//             .simplify()
//             .as_bool()
//             .unwrap());
//         // all zeroes
//         assert!(ctz()
//             .make_expression(&ctx, &vec![], &vec![bit_vec_from_u64(&ctx, 0, 32)], 32)
//             ._eq(&bit_vec_from_u64(&ctx, 32, 32))
//             .simplify()
//             .as_bool()
//             .unwrap());
//         // all ones
//         assert!(ctz()
//             .make_expression(
//                 &ctx,
//                 &vec![],
//                 &vec![z3::ast::BV::from_i64(&ctx, -1, 32)],
//                 32
//             )
//             ._eq(&bit_vec_from_u64(&ctx, 0, 32))
//             .simplify()
//             .as_bool()
//             .unwrap());
//         // 00 1010
//         assert!(ctz()
//             .make_expression(&ctx, &vec![], &vec![bit_vec_from_u64(&ctx, 10, 6)], 6)
//             ._eq(&bit_vec_from_u64(&ctx, 1, 6))
//             .simplify()
//             .as_bool()
//             .unwrap());
//     }

//     #[test]
//     fn clz_test() {
//         let _ = env_logger::try_init();
//         let cfg = z3::Config::new();
//         let ctx = z3::Context::new(&cfg);

//         // 0000 0000 0000 0000 0000 0000 0000 0010
//         assert!(clz()
//             .make_expression(&ctx, &vec![], &vec![bit_vec_from_u64(&ctx, 2, 32)], 32)
//             ._eq(&bit_vec_from_u64(&ctx, 30, 32))
//             .simplify()
//             .as_bool()
//             .unwrap());
//         // all zeroes
//         assert!(clz()
//             .make_expression(&ctx, &vec![], &vec![bit_vec_from_u64(&ctx, 0, 32)], 32)
//             ._eq(&bit_vec_from_u64(&ctx, 32, 32))
//             .simplify()
//             .as_bool()
//             .unwrap());
//         // all ones
//         assert!(clz()
//             .make_expression(
//                 &ctx,
//                 &vec![],
//                 &vec![z3::ast::BV::from_i64(&ctx, -1, 32)],
//                 32
//             )
//             ._eq(&bit_vec_from_u64(&ctx, 0, 32))
//             .simplify()
//             .as_bool()
//             .unwrap());
//         // 00 1010
//         assert!(clz()
//             .make_expression(&ctx, &vec![], &vec![bit_vec_from_u64(&ctx, 10, 6)], 6)
//             ._eq(&bit_vec_from_u64(&ctx, 2, 6))
//             .simplify()
//             .as_bool()
//             .unwrap());
//     }
// }
