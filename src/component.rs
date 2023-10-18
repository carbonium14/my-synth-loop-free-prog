use crate::{Id, Operator, Vecs};
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

    fn make_operator(&self, immediates: &Vec<Vecs<u64>>, operands: &[Id]) -> Operator;

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        // immediates: &[BitVec<'a>],
        // operands: &[BitVec<'a>],
        // immediates: &[Vec<BitVec<'a>>],
        // operands: &[Vec<BitVec<'a>>],

        immediates: &[Vecs<BitVec<'a>>],
        operands: &[Vecs<BitVec<'a>>],
        bit_width: u32,
    ) -> 
        //BitVec<'a> 
        Vecs<BitVec<'a>>;
        
    /// How many immediates does this component require?
    fn immediate_arity(&self) -> usize {
        0
    }
}

#[derive(Debug)]
struct Const([usize; 2]);

impl Component for Const {
    fn operand_arity(&self) -> usize {
        0
    }

    fn make_operator(&self, immediates: &Vec<Vecs<u64>>, _operands: &[Id]) -> Operator {
        Operator::Const(self.0)
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        // _immediates: &[Vec<BitVec<'a>>],
        // _operands: &[Vec<BitVec<'a>>],
        immediates: &[Vecs<BitVec<'a>>],
        operands: &[Vecs<BitVec<'a>>],
        bit_width: u32,
    ) -> Vecs<BitVec<'a>> {

        // if let Some(val) = self.0 {
        //     BitVec::from_i64(context, val as i64, bit_width)
        // } else {
        //     immediates[0][0].clone()
        // }

        let mut result : Vecs<BitVec<'a>> = Vecs::new({
            self.0
        });

        let dims = self.0;
        for i in 0 .. dims[0] {
            for j in 0 .. dims[1] {
                result.vecs[i as usize].push(BitVec::from_i64(context, 10 as i64, bit_width));
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


pub fn const_(val: [usize; 2]) -> Box<dyn Component> {
    Box::new(Const(val)) as _
}

#[derive(Debug)]
struct TfAbs;

impl Component for TfAbs {
    fn operand_arity(&self) -> usize {
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<u64>>, operands: &[Id]) -> Operator {
        Operator::TfAbs(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<BitVec<'a>>],
        operands: &[Vecs<BitVec<'a>>],
        bit_width: u32,
    ) -> Vecs<BitVec<'a>> {
        //TODO：目前只是二维,对于一维的数组，我们用x[1][m]表示长度为m的一维数组，他的dims = [1,m]

        // 取相同长度并且填充为0的数组，作取相反数的结果用
        let const0 = zero(context, bit_width);
        let sz = operands[0].dims;
        let mut result: Vecs<BitVec> = Vecs::new(operands[0].dims);
        // 它（for循环）是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
        // 所以这里是左闭右开区间
        for i in 0..sz[0] {
            for j in 0..sz[1] {
                 //计算每一个元素的相反数，作为后面的判断，如果是正数就直接返回，负数返回相反数
                let minus_num = const0.bvsub(&operands[0].vecs[i][j]);
                // 判断输入的数是正数还是负数
                let plus_or_minus = operands[0].vecs[i][j].bvslt(&const0).ite(&one(context, bit_width), &zero(context, bit_width));
                // 正数直接返回，负数返回相反数
                result.vecs[i].push(plus_or_minus._eq(&one(context, bit_width)).ite(&minus_num, &operands[0].vecs[i][j]))
            }   
        }
        return result;
    }
}

pub fn tf_abs() -> Box<dyn Component> {
    Box::new(TfAbs) as _
}

// #[derive(Debug)]
// struct TfAdd;

// impl Component for TfAdd {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfAdd(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 获取两个输入的长度
//         let size0 = operands[0].len();
//         let size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         // 如果输入等长，那么数组每个元素进行相加计算
//         if size0 == size1 {
//             for index in 0..size0 {
//                 result.push(operands[0][index].bvadd(&operands[1][index]));
//             }
//         } else { 
//             // 如果输入不等长，那么先把短的数组扩充到和长的数组一样长（填充0）然后再相加
//             let const0 = zero(context, bit_width);
//             if size0 < size1 {
//                 // 第一个输入比第二个输入短，先计算公共部分，然后第二个输入多出来的部分加上0即可
//                 for index in 0..size0 {
//                     result.push(operands[1][index].bvadd(&operands[0][index]));
//                 }
//                 for index in 0..(size1 - size0) {
//                     result.push(operands[1][index].bvadd(&const0));
//                 }
//             } else {
//                 // 第二个输入比第一个输入短，先计算公共部分，然后第一个输入多出来的部分加上0即可
//                 for index in 0..size1 {
//                     result.push(operands[0][index].bvadd(&operands[1][index]));
//                 }
//                 for index in 0..(size0 - size1) {
//                     result.push(operands[0][index].bvadd(&const0));
//                 }
//             }
//         }
//         return result;
//     }
// }

// pub fn tf_add() -> Box<dyn Component> {
//     Box::new(TfAdd) as _
// }

// #[derive(Debug)]
// struct TfMul;

// impl Component for TfMul {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfMul(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 获取两个输入的长度
//         let size0 = operands[0].len();
//         let size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         // 如果输入等长，那么数组每个元素进行相乘计算
//         if size0 == size1 {
//             for index in 0..size0 {
//                 result.push(operands[0][index].bvmul(&operands[1][index]));
//             }
//         } else { 
//             // 如果输入不等长，那么先把短的数组扩充到和长的数组一样长（填充1）然后再相乘
//             let const1 = one(context, bit_width);
//             if size0 < size1 {
//                 // 第一个输入比第二个输入短，先计算公共部分，然后第二个输入多出来的部分乘上1即可
//                 for index in 0..size0 {
//                     result.push(operands[1][index].bvmul(&operands[0][index]));
//                 }
//                 for index in 0..(size1 - size0) {
//                     result.push(operands[1][index].bvmul(&const1));
//                 }
//             } else {
//                 // 第二个输入比第一个输入短，先计算公共部分，然后第一个输入多出来的部分乘上1即可
//                 for index in 0..size1 {
//                     result.push(operands[0][index].bvmul(&operands[1][index]));
//                 }
//                 for index in 0..(size0 - size1) {
//                     result.push(operands[0][index].bvmul(&const1));
//                 }
//             }
//         }
//         return result;
//     }
// }

// pub fn tf_mul() -> Box<dyn Component> {
//     Box::new(TfMul) as _
// }

// #[derive(Debug)]
// struct TfDiv;

// impl Component for TfDiv {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfDiv(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 获取两个输入的长度
//         let size0 = operands[0].len();
//         let size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         // 如果输入等长，那么数组每个元素进行相除计算
//         if size0 == size1 {
//             for index in 0..size0 {
//                 result.push(operands[0][index].bvsdiv(&operands[1][index]));
//             }
//         } else { 
//             // 如果输入不等长，那么先把短的数组扩充到和长的数组一样长（填充1）然后再相除
//             let const1 = one(context, bit_width);
//             if size0 < size1 {
//                 // 第一个输入比第二个输入短，先计算公共部分，然后第二个输入多出来的部分除以1即可
//                 for index in 0..size0 {
//                     result.push(operands[1][index].bvsdiv(&operands[0][index]));
//                 }
//                 for index in 0..(size1 - size0) {
//                     result.push(operands[1][index].bvsdiv(&const1));
//                 }
//             } else {
//                 // 第二个输入比第一个输入短，先计算公共部分，然后第一个输入多出来的部分除以1即可
//                 for index in 0..size1 {
//                     result.push(operands[0][index].bvsdiv(&operands[1][index]));
//                 }
//                 for index in 0..(size0 - size1) {
//                     result.push(operands[0][index].bvsdiv(&const1));
//                 }
//             }
//         }
//         return result;
//     }
// }

// pub fn tf_div() -> Box<dyn Component> {
//     Box::new(TfDiv) as _
// }

// #[derive(Debug)]
// struct TfArgMax;

// impl Component for TfArgMax {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfArgMax(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 寻找最大值的下标，就是遍历数组，然后依次比较出最大值，再找到对应的下标即可
//         let size = operands[0].len();
//         // 记录下标，初始值设为0，这个0要用bitvec版本的0
//         let mut ans = zero(context, bit_width);
//         // 记录目标值，为了能和“最大”比较，初始值应该设为最小值，当然也得是bitvec版本的值
//         let mut val = zero(context, bit_width);
//         // 淦！rust不能像其他语言那样for循环的时候同时取到下标和值，所以得遍历两遍
//         for index in 0..size {
//             // 你说for里面用下标，循环体用下标取值？不好意思，ast提供的函数不允许同时返回两个值，所以处理下标和值得分开来
//             val = operands[0][index].bvsgt(&val).ite(&operands[0][index], &val);
//         }
//         for index in 0..size {
//             // 先遍历值，然后根据值确定下标就遍历看哪个相等就可以了，注意值是bitvec版本的值
//             ans = operands[0][index]._eq(&val).ite(&BitVec::from_i64(context, index as i64, bit_width), &zero(context, bit_width))
//         }
//         // 目前的结构，必须要返回一个和输入长度相等的数组，否则会报错！！！
//         let mut result: Vec<BitVec> = Vec::new();
//         for _index in 0..size {
//             // 我这边费老大劲找如何解决use moved value的问题，改了好多地方都不行，结果一个clone就解决了？？？？
//             // 淦！我感觉我的时间都浪费在这上面了！！！
//             result.push(ans.clone());
//         }
//         return result;
//     }
// }

// pub fn tf_argmax() -> Box<dyn Component> {
//     Box::new(TfArgMax) as _
// }

// #[derive(Debug)]
// struct TfArgMin;

// impl Component for TfArgMin {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfArgMin(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 寻找最大值的下标，就是遍历数组，然后依次比较出最小值，再找到对应的下标即可
//         let size = operands[0].len();
//         // 记录下标，初始值设为最大值，这个最大值要用bitvec版本的最大值
//         let mut ans = BitVec::from_i64(context, 9223372036854775807, bit_width);
//         // 记录目标值，为了能和“最小”比较，初始值应该设为最大值，当然也得是bitvec版本的值
//         let mut val = zero(context, bit_width);
//         // 淦！rust不能像其他语言那样for循环的时候同时取到下标和值，所以得遍历两遍
//         for index in 0..size {
//             // 你说for里面用下标，循环体用下标取值？不好意思，ast提供的函数不允许同时返回两个值，所以处理下标和值得分开来
//             val = operands[0][index].bvslt(&val).ite(&operands[0][index], &val);
//         }
//         for index in 0..size {
//             // 先遍历值，然后根据值确定下标就遍历看哪个相等就可以了，注意值是bitvec版本的值
//             ans = operands[0][index]._eq(&val).ite(&BitVec::from_i64(context, index as i64, bit_width), &zero(context, bit_width))
//         }
//         // 目前的结构，必须要返回一个和输入长度相等的数组，否则会报错！！！
//         let mut result: Vec<BitVec> = Vec::new();
//         for _index in 0..size {
//             result.push(ans.clone());
//         }
//         return result;
//     }
// }

// pub fn tf_argmin() -> Box<dyn Component> {
//     Box::new(TfArgMin) as _
// }

// #[derive(Debug)]
// struct TfBooleanMask;

// impl Component for TfBooleanMask {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfBooleanMask(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 获取两个输入的长度，等长后进行后续操作
//         let size0 = operands[0].len();
//         let size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         // 填充物，如果掩码部分不是1，那么就返回0
//         let const0 = zero(context, bit_width);
//         if size0 != size1 {
//             // TODO: 这里可以报错，或者干别的
//         } else {
//             for index in 0..size0 {
//                 //如果掩码为1，则返回自身，如果掩码为0，则返回0
//                 result.push(operands[1][index]._eq(&one(context, bit_width)).ite(&operands[0][index], &const0));
//             }
//         }
//         return result;
//     }
// }

// pub fn tf_boolean_mask() -> Box<dyn Component> {
//     Box::new(TfBooleanMask) as _
// }

// #[derive(Debug)]
// struct TfCast;

// impl Component for TfCast {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfCast(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         _bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 类型转换之后返回
//         // 只不过目前还没有类型，所以直接返回就行
//         return operands[0].to_vec();
//     }
// }

// pub fn tf_cast() -> Box<dyn Component> {
//     Box::new(TfCast) as _
// }

// #[derive(Debug)]
// struct TfClipByValue;

// impl Component for TfClipByValue {
//     fn operand_arity(&self) -> usize {
//         3
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfClipByValue(operands[0], operands[1], operands[2])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 第一个是输入的数组，第二个是最小值，第三个是最大值
//         // 要求数组里每一项都要和最大值和最小值比较，在此范围（含最大值和最小值）之外的，小的换成最小值，大的换成最大值
//         let size = operands[0].len();
//         // 目前仅有var数组的情况，还不清楚会不会单独搞一个var变量，先用数组的方法取值即可
//         let min_value = &operands[1][0];
//         let max_value = &operands[2][0];
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size {
//             // 判断当前值是否小于等于最小值，当前值是否大于等于最大值，1则为成立，0则为不成立
//             let is_min = operands[0][index].bvsle(&min_value).ite(&one(context, bit_width), &zero(context, bit_width));
//             let is_max = operands[0][index].bvsge(&max_value).ite(&one(context, bit_width), &zero(context, bit_width));
//             // 先判断是不是比最小值小，如果是则取最小值，再比较是不是比最大值大，如果是则取最大值，其余情况原值返回
//             result.push(operands[0][index]._eq(&is_min).ite(&min_value, &operands[0][index]._eq(&is_max).ite(&max_value, &operands[0][index])));
//         }
//         return result;
//     }
// }

// pub fn tf_clip_by_value() -> Box<dyn Component> {
//     Box::new(TfClipByValue) as _
// }

// #[derive(Debug)]
// struct TfEqual;

// impl Component for TfEqual {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfEqual(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 依次比较两个数组每个元素是否相等即可
//         // TODO：名义上如果不等长要考虑广播，下一步需要考虑不等长的情况
//         let size0 = operands[0].len();
//         let _size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size0 {
//             result.push(operands[0][index]._eq(&operands[1][index]).ite(&one(context, bit_width), &zero(context, bit_width)));
//         }
//         return result;
//     }
// }

// pub fn tf_equal() -> Box<dyn Component> {
//     Box::new(TfEqual) as _
// }

// #[derive(Debug)]
// struct TfFill;

// impl Component for TfFill {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfFill(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         _bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 根据第一个输入的长度填充第二个数
//         // 不过吧。。。具体实现的话，既然数组长度已经通过其他途径确定了，这第一个量有何用。。。
//         // 而且吧。。。咱所有的输入都是数组，那第二个输入可以看成一个数组，直接返回不就好了。。。
//         return operands[1].to_vec();
//         /* 好吧，这一段应该是想象中的填充，只不过第一个变量里面的长度不一定是最终的长度
//         let length = &operands[0][0];
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..length {
//             result.push(operands[1][index])
//         }
//         return result;
//         */
//     }
// }

// pub fn tf_fill() -> Box<dyn Component> {
//     Box::new(TfFill) as _
// }

// #[derive(Debug)]
// struct TfGreater;

// impl Component for TfGreater {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfGreater(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 依次比较两个数组每个元素是否大于即可
//         // TODO：名义上如果不等长要考虑广播，下一步需要考虑不等长的情况
//         let size0 = operands[0].len();
//         let _size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size0 {
//             result.push(operands[0][index].bvsgt(&operands[1][index]).ite(&one(context, bit_width), &zero(context, bit_width)));
//         }
//         return result;
//     }
// }

// pub fn tf_greater() -> Box<dyn Component> {
//     Box::new(TfGreater) as _
// }

// #[derive(Debug)]
// struct TfGreaterEqual;

// impl Component for TfGreaterEqual {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfGreaterEqual(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 依次比较两个数组每个元素是否大于等于即可
//         // TODO：名义上如果不等长要考虑广播，下一步需要考虑不等长的情况
//         let size0 = operands[0].len();
//         let _size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size0 {
//             result.push(operands[0][index].bvsge(&operands[1][index]).ite(&one(context, bit_width), &zero(context, bit_width)));
//         }
//         return result;
//     }
// }

// pub fn tf_greater_equal() -> Box<dyn Component> {
//     Box::new(TfGreaterEqual) as _
// }

// #[derive(Debug)]
// struct TfNotEqual;

// impl Component for TfNotEqual {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfNotEqual(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 依次比较两个数组每个元素是否大于不等于即可
//         // TODO：名义上如果不等长要考虑广播，下一步需要考虑不等长的情况
//         let size0 = operands[0].len();
//         let _size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size0 {
//             result.push(operands[0][index].bvsge(&operands[1][index]).ite(&zero(context, bit_width), &one(context, bit_width)));
//         }
//         return result;
//     }
// }

// pub fn tf_not_equal() -> Box<dyn Component> {
//     Box::new(TfNotEqual) as _
// }

// #[derive(Debug)]
// struct TfNegative;

// impl Component for TfNegative {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfNegative(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 依次遍历每个数，取相反数即可
//         // 相反数可以采用常数零减去当前数的方法
//         let const0 = zero(context, bit_width);
//         let size = operands[0].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size {
//             result.push(const0.bvsub(&operands[0][index]));
//         }
//         return result;
//     }
// }

// pub fn tf_negative() -> Box<dyn Component> {
//     Box::new(TfNegative) as _
// }

// #[derive(Debug)]
// struct TfReciprocal;

// impl Component for TfReciprocal {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfReciprocal(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 依次遍历每个数，取倒数即可
//         // 倒数可以采用常数1除以当前数的方法
//         let const1 = one(context, bit_width);
//         let size = operands[0].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size {
//             result.push(const1.bvsdiv(&operands[0][index]));
//         }
//         return result;
//     }
// }

// pub fn tf_reciprocal() -> Box<dyn Component> {
//     Box::new(TfReciprocal) as _
// }

// #[derive(Debug)]
// struct TfCountNonzero;

// impl Component for TfCountNonzero {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfCountNonzero(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 遍历输入，找出非0的数并统计个数即可
//         let size = operands[0].len();
//         let mut ans = zero(context, bit_width);
//         for index in 0..size {
//             // 为了方便计数，可以用z3判断是不是为0，然后用z3的方式相加
//             let is_zero = operands[0][index]._eq(&zero(context, bit_width)).ite(&zero(context, bit_width),&one(context, bit_width));
//             ans = ans.bvadd(&is_zero);
//         }
//         // 还是要保持返回的数组长度和输入的长度一致
//         let mut result: Vec<BitVec> = Vec::new();
//         for _index in 0..size {
//             result.push(ans.clone());
//         }
//         return result;
//     }
// }

// pub fn tf_count_nonzero() -> Box<dyn Component> {
//     Box::new(TfCountNonzero) as _
// }

// #[derive(Debug)]
// struct TfCumsum;

// impl Component for TfCumsum {
//     fn operand_arity(&self) -> usize {
//         3
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfCumsum(operands[0], operands[1], operands[2])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 第一个是输入的数组，第三个是判断是正序还是倒序
//         // 第二个是判断算不算第一个，加入一个数组[a, b, c]，如果第二个参数是真，那么结果是[a, a + b, a + b + c]，否则就是[0, a, a + b]
//         let size = operands[0].len();
//         let mut ans = zero(context, bit_width);
//         let mut result: Vec<BitVec> = Vec::new();
//         // 1为true，0为false
//         let is_add_first = operands[1][0]._eq(&one(context, bit_width)).ite(&one(context, bit_width),&zero(context, bit_width));
//         let is_reverse = operands[2][0]._eq(&one(context, bit_width)).ite(&one(context, bit_width),&zero(context, bit_width));
//         if is_add_first == zero(context, bit_width) {
//             for index in 0..size {
//                 result.push(ans.bvadd(&operands[0][index]));
//                 ans = ans.bvadd(&operands[0][index]);
//             }
//         } else {
//             result.push(zero(context, bit_width));
//             for index in 0..size - 1 {
//                 result.push(ans.bvadd(&operands[0][index]));
//                 ans = ans.bvadd(&operands[0][index]);
//             }
//         }
//         if is_reverse == one(context, bit_width) {
//             result.reverse();
//         }
//         return result;
//     }
// }

// pub fn tf_cumsum() -> Box<dyn Component> {
//     Box::new(TfCumsum) as _
// }

// #[derive(Debug)]
// struct TfMaximum;

// impl Component for TfMaximum {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfMaximum(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         _bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 遍历比较求出最大的那个并放入结果中即可
//         // TODO：由于限制只要求长度一致，还要考虑长度不一致的情况
//         let size0 = operands[0].len();
//         let _size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size0 {
//             result.push(operands[0][index].bvsgt(&operands[1][index]).ite(&operands[0][index], &operands[1][index]));
//         }
//         return result;
//     }
// }

// pub fn tf_maximum() -> Box<dyn Component> {
//     Box::new(TfMaximum) as _
// }

// #[derive(Debug)]
// struct TfMinimum;

// impl Component for TfMinimum {
//     fn operand_arity(&self) -> usize {
//         2
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfMinimum(operands[0], operands[1])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         _bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 遍历比较求出最小的那个并放入结果中即可
//         // TODO：由于限制只要求长度一致，还要考虑长度不一致的情况
//         let size0 = operands[0].len();
//         let _size1 = operands[1].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size0 {
//             result.push(operands[0][index].bvslt(&operands[1][index]).ite(&operands[0][index], &operands[1][index]));
//         }
//         return result;
//     }
// }

// pub fn tf_minimum() -> Box<dyn Component> {
//     Box::new(TfMinimum) as _
// }

// #[derive(Debug)]
// struct TfReverse;

// impl Component for TfReverse {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfReverse(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         _bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 倒序，就是把第size - index个放到index下标里
//         let size = operands[0].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size {
//             result.push(operands[0][size - index].clone());
//         }
//         return result;
//     }
// }

// pub fn tf_reverse() -> Box<dyn Component> {
//     Box::new(TfReverse) as _
// }

// #[derive(Debug)]
// struct TfSign;

// impl Component for TfSign {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfSign(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 确定每个数字的正负，正数1、负数-1、0为0
//         let size = operands[0].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         // 手动定义1、-1、0，只不过-1没有函数，得自己用方法设定
//         let plus_one = one(context, bit_width);
//         let zero = zero(context, bit_width);
//         let minus_one = BitVec::from_i64(context, -1, bit_width);
//         for index in 0..size {
//             // 先判断是否为负，是则返回-1，否则判断是否为正，是则返回1，否则不是正数也不是负数，则为0
//             result.push(operands[0][index].bvslt(&zero).ite(&minus_one, &operands[0][index].bvsgt(&zero).ite(&plus_one, &zero)));
//         }
//         return result;
//     }
// }

// pub fn tf_sign() -> Box<dyn Component> {
//     Box::new(TfSign) as _
// }

// #[derive(Debug)]
// struct TfSquare;

// impl Component for TfSquare {
//     fn operand_arity(&self) -> usize {
//         1
//     }

//     fn make_operator(&self, _immediates: &Vec<Vec<u64>>, operands: &[Id]) -> Operator {
//         Operator::TfSquare(operands[0])
//     }

//     fn make_expression<'a>(
//         &self,
//         _context: &'a z3::Context,
//         _immediates: &[Vec<BitVec<'a>>],
//         operands: &[Vec<BitVec<'a>>],
//         _bit_width: u32,
//     ) -> Vec<BitVec<'a>> {
//         // 遍历每个元素，返回每个元素的平方即可
//         let size = operands[0].len();
//         let mut result: Vec<BitVec> = Vec::new();
//         for index in 0..size {
//             // 平方就是自己乘以自己
//             result.push(operands[0][index].bvmul(&operands[0][index]));
//         }
//         return result;
//     }
// }

// pub fn tf_square() -> Box<dyn Component> {
//     Box::new(TfSquare) as _
// }

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
                let $c = Const(*c);
                $body
            }
            Operator::TfAbs(_) => {
                let $c = TfAbs;
                $body
            }
            // Operator::TfAdd(_, _) => {
            //     let $c = TfAdd;
            //     $body
            // }
            // Operator::TfMul(_, _) => {
            //     let $c = TfMul;
            //     $body
            // }
            // Operator::TfDiv(_, _) => {
            //     let $c = TfDiv;
            //     $body
            // }
            // Operator::TfArgMax(_) => {
            //     let $c = TfArgMax;
            //     $body
            // }
            // Operator::TfArgMin(_) => {
            //     let $c = TfArgMin;
            //     $body
            // }
            // Operator::TfBooleanMask(_, _) => {
            //     let $c = TfBooleanMask;
            //     $body
            // }
            // Operator::TfCast(_) => {
            //     let $c = TfCast;
            //     $body
            // }
            // Operator::TfClipByValue(_, _, _) => {
            //     let $c = TfClipByValue;
            //     $body
            // }
            // Operator::TfEqual(_, _) => {
            //     let $c = TfEqual;
            //     $body
            // }
            // Operator::TfFill(_, _) => {
            //     let $c = TfFill;
            //     $body
            // }
            // Operator::TfGreater(_, _) => {
            //     let $c = TfGreater;
            //     $body
            // }
            // Operator::TfGreaterEqual(_, _) => {
            //     let $c = TfGreaterEqual;
            //     $body
            // }
            // Operator::TfNotEqual(_, _) => {
            //     let $c = TfNotEqual;
            //     $body
            // }
            // Operator::TfNegative(_) => {
            //     let $c = TfNegative;
            //     $body
            // }
            // Operator::TfReciprocal(_) => {
            //     let $c = TfReciprocal;
            //     $body
            // }
            // Operator::TfCountNonzero(_) => {
            //     let $c = TfCountNonzero;
            //     $body
            // }
            // Operator::TfCumsum(_, _, _) => {
            //     let $c = TfCumsum;
            //     $body
            // }
            // Operator::TfMaximum(_, _) => {
            //     let $c = TfMaximum;
            //     $body
            // }
            // Operator::TfMinimum(_, _) => {
            //     let $c = TfMinimum;
            //     $body
            // }
            // Operator::TfReverse(_) => {
            //     let $c = TfReverse;
            //     $body
            // }
            // Operator::TfSign(_) => {
            //     let $c = TfSign;
            //     $body
            // }
            // Operator::TfSquare(_) => {
            //     let $c = TfSquare;
            //     $body
            // }
        }
    };
}

impl Component for Operator {
    fn operand_arity(&self) -> usize {
        Operator::arity(self)
    }

    fn make_operator(&self, immediates: &Vec<Vecs<u64>>, operands: &[Id]) -> Operator {
        with_operator_component!(self, |c| c.make_operator(immediates, operands))
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        immediates: &[Vecs<BitVec<'a>>],
        operands: &[Vecs<BitVec<'a>>],
        bit_width: u32,
    ) -> Vecs<BitVec<'a>> {
        with_operator_component!(self, |c| {
            c.make_expression(context, immediates, operands, bit_width)
        })
    }

    fn immediate_arity(&self) -> usize {
        with_operator_component!(self, |c| c.immediate_arity())
    }
}

