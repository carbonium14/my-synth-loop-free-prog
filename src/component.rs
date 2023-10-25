use crate::{Id, Operator, Vecs, Type};
use std::{fmt::Debug, ops::Sub, ops::Add, ops::Mul, usize, collections::HashMap};
use z3::ast::{Ast, Int};

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
fn Int_from_u64(context: &z3::Context, val: u64, bit_width: u32) -> Int {
    Int::from_i64(context, val as i64)
}

fn zero(context: &z3::Context, bit_width: u32) -> Int {
    Int_from_u64(context, 0, bit_width)
}

fn one(context: &z3::Context, bit_width: u32) -> Int {
    Int_from_u64(context, 1, bit_width)
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

#[derive(Debug)]
struct Const([usize; 2]);

impl Component for Const {
    fn operand_arity(&self) -> usize {
        0
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, _operands: &[Id]) -> Operator {
        Operator::Const(self.0)
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


        let mut result : Vecs<Int<'a>> = Vecs::new({
            self.0
        });

        let dims = self.0;
        for i in 0 .. dims[0] {
            for _j in 0 .. dims[1] {
                result.vecs[i as usize].push(Int::from_i64(context, 10 as i64));
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfAbs(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        //TODO：目前只是二维,对于一维的数组，我们用x[1][m]表示长度为m的一维数组，他的dims = [1,m]

        // 取相同长度并且填充为0的数组，作取相反数的结果用
        let const0 = zero(context, bit_width);
        let sz = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(operands[0].dims);
        // 它（for循环）是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
        // 所以这里是左闭右开区间
        for i in 0..sz[0] {
            for j in 0..sz[1] {
                //计算每一个元素的相反数，作为后面的判断，如果是正数就直接返回，负数返回相反数
                let minus_num = const0.clone().sub(&operands[0].vecs[i][j]);
                // 判断输入的数是正数还是负数
                let plus_or_minus = operands[0].vecs[i][j].lt(&const0).ite(&one(context, bit_width), &zero(context, bit_width));
                // 正数直接返回，负数返回相反数
                result.vecs[i].push(plus_or_minus._eq(&one(context, bit_width)).ite(&minus_num, &operands[0].vecs[i][j]))
                //
                //operands[0].vecs[i][j].lt(&const0).ite(Int::<'a>::sub(context, ))
            }   
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfAdd(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 获取两个输入的长度
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        // 取两个数组维度的最大值
        // 不过rust连min、max函数都不给！！！还得我自己手动去实现！！！
        // 服了，连三元表达式都没有！！！
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        // 按理说其余情况应该报错，只不过需不需要显示地提出就要看需求了
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        // 如果两个数组维度和长度相同，那么遍历然后直接相加即可，否则得先扩充然后再实现
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j].clone().add(&operands[1].vecs[i][j]));
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfMul(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 获取两个输入的长度
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        // 取两个数组维度的最大值
        // 不过rust连min、max函数都不给！！！还得我自己手动去实现！！！
        // 服了，连三元表达式都没有！！！
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        // 如果两个数组维度和长度相同，那么遍历然后直接相乘即可，否则得先扩充然后再实现
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j].clone().mul(&operands[1].vecs[i][j]));
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfDiv(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 获取两个输入的长度
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        // 取两个数组维度的最大值
        // 不过rust连min、max函数都不给！！！还得我自己手动去实现！！！
        // 服了，连三元表达式都没有！！！
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        // 如果两个数组维度和长度相同，那么遍历然后直接相除即可，否则得先扩充然后再实现
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j].div(&operands[1].vecs[i][j]));
            }
        }
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
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfArgMax(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 寻找最大值的下标，就是遍历数组，然后依次比较出最大值，再找到对应的下标即可
        let size = operands[0].dims;
        // 最终返回的是个数组，元素的个数和第一个维度相同
        let mut ans: Vec<Int> = Vec::new();
        // 记录目标值，为了能和“最大”比较，初始值应该设为最小值，当然也得是bitvec版本的值
        #[allow(unused_assignments)]
        let mut val = zero(context, bit_width);
        // 淦！rust不能像其他语言那样for循环的时候同时取到下标和值，所以得遍历两遍
        for i in 0..size[0] {
            // 重置操作
            val = zero(context, bit_width);
            for j in 0..size[1] {
                // 你说for里面用下标，循环体用下标取值？不好意思，ast提供的函数不允许同时返回两个值，所以处理下标和值得分开来
                val = operands[0].vecs[i][j].gt(&val).ite(&operands[0].vecs[i][j], &val);
            }
            // 记录最终结果的下标
            let mut res = zero(context, bit_width);
            for j in 0..size[1] {
                // 先遍历值，然后根据值确定下标就遍历看哪个相等就可以了，注意值是bitvec版本的值
                res = operands[0].vecs[i][j]._eq(&val).ite(&Int::from_i64(context, i as i64), &zero(context, bit_width));
            }
            ans.push(res);
        }
        let mut result: Vecs<Int> = Vecs::new([1, size[0]]);
        for i in 0..size[0] {
            result.vecs[0].push(ans[i].clone());
        }
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
        1
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfArgMin(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 寻找最小值的下标，就是遍历数组，然后依次比较出最小值，再找到对应的下标即可
        let size = operands[0].dims;
        // 最终返回的是个数组，元素的个数和第一个维度相同
        let mut ans: Vec<Int> = Vec::new();
        // 记录目标值，为了能和“最小”比较，初始值应该设为最大值，当然也得是bitvec版本的值
        #[allow(unused_assignments)]
        let mut val = Int::from_i64(context, 9223372036854775807);
        // 淦！rust不能像其他语言那样for循环的时候同时取到下标和值，所以得遍历两遍
        for i in 0..size[0] {
            // 重置操作
            val = Int::from_i64(context, 9223372036854775807);
            for j in 0..size[1] {
                // 你说for里面用下标，循环体用下标取值？不好意思，ast提供的函数不允许同时返回两个值，所以处理下标和值得分开来
                val = operands[0].vecs[i][j].lt(&val).ite(&operands[0].vecs[i][j], &val);
            }
            // 记录最终结果的下标
            let mut res = zero(context, bit_width);
            for j in 0..size[1] {
                // 先遍历值，然后根据值确定下标就遍历看哪个相等就可以了，注意值是bitvec版本的值
                res = operands[0].vecs[i][j]._eq(&val).ite(&Int::from_i64(context, i as i64), &zero(context, bit_width));
            }
            ans.push(res);
        }
        let mut result: Vecs<Int> = Vecs::new([1, size[0]]);
        for i in 0..size[0] {
            result.vecs[0].push(ans[i].clone());
        }
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
        // 获取两个输入的长度，等长后进行后续操作
        // mask（也就是第二个参数）的维度可以是和数入数组等维度，也可以是维度少一维
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let mut result: Vecs<Int> = Vecs::new(size0);
        // 填充物，如果掩码部分不是1，那么就返回0
        let const0 = zero(context, bit_width);
        // 保证两个长度相等，否则报错
        for i in 0..size0[0] {
            for j in 0..size0[1] {
                // 掩码是二维数组，那就依次遍历
                if size1[0] != 1 {
                    //如果掩码为1，则返回，如果掩码为0，则不返回
                    let ans = operands[1].vecs[i][j]._eq(&one(context, bit_width)).ite(&operands[0].vecs[i][j], &const0);
                    // 注意，为0的部分不要加入到数组里面
                    if ans != const0 {
                        result.vecs[i].push(ans);
                    }
                // 掩码是一维数组，那就一维数组里面每个元素对应输入的第一维度
                } else {
                    let ans = operands[1].vecs[i][0]._eq(&one(context, bit_width)).ite(&operands[0].vecs[i][j], &const0);
                    if ans != const0 {
                        result.vecs[i].push(ans);
                    }
                }
            }
        }
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
        // 类型转换之后返回
        // 只不过目前还没有类型，所以直接返回就行
        return operands[0].clone();
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfConcat(operands[0], operands[1], operands[2])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 这个表达式本来要做的是把所有要拼接的数组放在一个数组里面，但是这样的话就会导致数组维度不对
        // 所以在这里我们把数组展开，由于大部分样例只需要两个数组，所以前两个就是数组，第三个是轴
        // 虽然轴的取值多样，但是在这里由于维度限制在了二维，所以轴取0、1、-1即可，而-1和1在二维下是一样的
        // 轴为0就是按行遍历把每一行放进去，轴为1就是按列遍历把每一列放进去
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let axis = operands[2].vecs[0][0].clone();
        // 保证输入的维度和长度相等，所以轴为0的时候最终的行数为行数之和，列数不变，轴为1的时候行数不变，最终的列数为列数之和
        let row = if axis == zero(context, bit_width) {
            size0[0] + size1[0]
        } else {
            size0[0]
        };
        let col = if axis == zero(context, bit_width) {
            size0[1]
        } else {
            size0[1] + size1[1]
        };
        let mut result: Vecs<Int> = Vecs::new([row, col]);
        // 轴为0的时候遍历行，把每一行的东西放进去，轴为1的时候遍历行和列，同样行的内容里面放入列的数据
        if axis == zero(context, bit_width) {
            for i in 0..size0[0] {
                result.vecs.push(operands[0].vecs[i].clone());
            }
            for i in 0..size1[0] {
                result.vecs.push(operands[1].vecs[i].clone());
            }
        } else {
            for i in 0..size0[0] {
                for j in 0..size0[1] {
                    result.vecs[i].push(operands[0].vecs[i][j].clone());
                }
            }
            for i in 0..size1[0] {
                for j in 0..size1[1] {
                    result.vecs[i].push(operands[1].vecs[i][j].clone());
                }
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfClipByValue(operands[0], operands[1], operands[2])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 第一个是输入的数组，第二个是最小值，第三个是最大值
        // 要求数组里每一项都要和最大值和最小值比较，在此范围（含最大值和最小值）之外的，小的换成最小值，大的换成最大值
        let size = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(size);
        for i in 0..size[0] {
            // 对于数组而言，是最大最小数组上的一个值对应原数组的一个维度
            // 比如[[1, 2], [3, 4]]和[5, 6]，5对应的是[1, 2]，6对应的是[3, 4]
            let min_value = operands[1].vecs[i][0].clone();
            let max_value = operands[2].vecs[i][0].clone();
            for j in 0..size[1] {
                // 判断当前值是否小于等于最小值，当前值是否大于等于最大值，1则为成立，0则为不成立
                let is_min = operands[0].vecs[i][j].le(&min_value).ite(&one(context, bit_width), &zero(context, bit_width));
                let is_max = operands[0].vecs[i][j].ge(&max_value).ite(&one(context, bit_width), &zero(context, bit_width));
                // 先判断是不是比最小值小，如果是则取最小值，再比较是不是比最大值大，如果是则取最大值，其余情况原值返回
                result.vecs[i].push(one(context, bit_width)._eq(&is_min).ite(&min_value, &one(context, bit_width)._eq(&is_max).ite(&max_value, &operands[0].vecs[i][j])));
            }
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
        // 依次比较两个数组每个元素是否相等即可
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j]._eq(&operands[1].vecs[i][j]).ite(&one(context, bit_width), &zero(context, bit_width)));
            }
        }
        return result;
    }
}

pub fn tf_equal() -> Box<dyn Component> {
    Box::new(TfEqual) as _
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
        // 根据输入的行数（第一个参数）和列数（第二个参数）来生成与单位矩阵中1位置相同的二维数组
        // 先都初始化为0，然后把单位矩阵位置上的数替换为1即可
        // 要注意接受的参数是常数，所以得根据输入的数组里面提取出常数
        // 要注意有个as方法，可以把bitvec里面的东西变成编程语言里面的数，它是一个option，所以要用match进行转换
        let row_option = operands[0].vecs[0][0].as_u64();
        #[allow(unused_assignments)]
        let mut row = 0;
        match row_option {
            Some(u) => row = u,
            None => row = 0,
        }
        let col_option = operands[1].vecs[0][0].as_u64();
        #[allow(unused_assignments)]
        let mut col = 0;
        match col_option {
            Some(u) => col = u,
            None => col = 0,
        }
        let mut result: Vecs<Int> = Vecs::new([row as usize, col as usize]);
        // 初始填充0
        for i in 0..row as usize {
            for j in 0..col as usize {
                result.vecs[i][j] = zero(context, bit_width);
            }
        }
        // 要设置最小的维度来填充1，其余的都是0
        let min_dim = if row > col {
            col
        } else {
            row
        } as usize;
        for i in 0..min_dim {
            result.vecs[i][i] = one(context, bit_width);
        }
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
        // 根据输入的行数和列数来生成全为1的二维数组
        // 要注意接受的参数是常数，所以得根据输入的数组里面提取出常数
        // 要注意有个as方法，可以把bitvec里面的东西变成编程语言里面的数，它是一个option，所以要用match进行转换
        let row_option = operands[0].vecs[0][0].as_u64();
        #[allow(unused_assignments)]
        let mut row = 0;
        match row_option {
            Some(u) => row = u,
            None => row = 0,
        }
        let col_option = operands[0].vecs[0][1].as_u64();
        #[allow(unused_assignments)]
        let mut col = 0;
        match col_option {
            Some(u) => col = u,
            None => col = 0,
        }
        let mut result: Vecs<Int> = Vecs::new([row as usize, col as usize]);
        for i in 0..row as usize {
            for j in 0..col as usize {
                result.vecs[i][j] = one(context, bit_width);
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
        // 根据输入的行数和列数来生成全为0的二维数组
        // 要注意接受的参数是常数，所以得根据输入的数组里面提取出常数
        // 要注意有个as方法，可以把bitvec里面的东西变成编程语言里面的数，它是一个option，所以要用match进行转换
        let row_option = operands[0].vecs[0][0].as_u64();
        #[allow(unused_assignments)]
        let mut row = 0;
        match row_option {
            Some(u) => row = u,
            None => row = 0,
        }
        let col_option = operands[0].vecs[0][1].as_u64();
        #[allow(unused_assignments)]
        let mut col = 0;
        match col_option {
            Some(u) => col = u,
            None => col = 0,
        }
        let mut result: Vecs<Int> = Vecs::new([row as usize, col as usize]);
        for i in 0..row as usize {
            for j in 0..col as usize {
                result.vecs[i][j] = zero(context, bit_width);
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfOnesLike(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 把输入的内容全部替换成1
        let size = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(size);
        for i in 0..size[0]{
            for j in 0..size[1] {
                result.vecs[i][j] = one(context, bit_width);
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfZerosLike(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 把输入的内容全部替换成0
        let size = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(size);
        for i in 0..size[0]{
            for j in 0..size[1] {
                result.vecs[i][j] = zero(context, bit_width);
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfFill(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 根据第一个输入的长度数组填充第二个数
        let length_x_option = operands[0].vecs[0][0].as_u64();
        let length_y_option = operands[0].vecs[0][1].as_u64();
        #[allow(unused_assignments)]
        let mut length_x = 0;
        match length_x_option {
            Some(l) => length_x = l,
            None => length_x = 0,
        }
        #[allow(unused_assignments)]
        let mut length_y = 0;
        match length_y_option {
            Some(l) => length_y = l,
            None => length_y = 0,
        }
        let mut result: Vecs<Int> = Vecs::new([length_x as usize, length_y as usize]);
        for i in 0..length_x as usize {
            for _j in 0..length_y {
                result.vecs[i].push(operands[1].vecs[0][0].clone());
            }
        }
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
        // 依次比较两个数组每个元素是否大于即可
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j].gt(&operands[1].vecs[i][j]).ite(&one(context, bit_width), &zero(context, bit_width)));
            }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfGreaterEqual(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 依次比较两个数组每个元素是否大于等于即可
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j].ge(&operands[1].vecs[i][j]).ite(&one(context, bit_width), &zero(context, bit_width)));
            }
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
        // 依次比较两个数组每个元素是否不等于即可
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j]._eq(&operands[1].vecs[i][j]).ite(&zero(context, bit_width), &one(context, bit_width)));
            }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfNegative(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 依次遍历每个数，取相反数即可
        // 相反数可以采用常数零减去当前数的方法
        let const0 = zero(context, bit_width);
        let size = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(size);
        for i in 0..size[0] {
            for j in 0..size[1] {
                result.vecs[i].push(const0.clone().sub(&operands[0].vecs[i][j]));
            }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReciprocal(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 依次遍历每个数，取倒数即可
        // 倒数可以采用常数1除以当前数的方法
        let const1 = one(context, bit_width);
        let size = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(size);
        for i in 0..size[0] {
            for j in 0..size[1] {
                result.vecs[i].push(const1.div(&operands[0].vecs[i][j]));
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfBincount(operands[0], operands[1], operands[2], operands[3])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 第一个输入是value数组，第二个输入是权重weight数组，第三个输入是最小长度，第四个输入是最大长度
        // 遍历每行，每行内部按照位置乘以权重然后相加，放入到对应的位置上
        let size0 = operands[0].dims;
        // 比最小长度小的会用0填充，比最大长度大的会忽略
        let min = operands[2].vecs[0][0].clone();
        let max = operands[3].vecs[0][0].clone();
        // 记录最大长度的值，所有维度都要设置为最大长度
        let mut max_length = zero(context, bit_width);
        // 记录每行的权重求和
        let mut hashmap_arr: Vec<HashMap<Int, Int>> = Vec::new();
        for i in 0..size0[0] {
            // 记录下每一个value对应的权重累加和
            let mut hashmap: HashMap<Int, Int> = HashMap::new();
            hashmap.clear();
            // 记录下每一维度的最大长度
            let mut maxlen = zero(context, bit_width);
            for j in 0..size0[1] {
                let value_weight = operands[0].vecs[i][j].clone().mul(&operands[1].vecs[i][j]);
                // rust的hashmap机制比较特殊，可以对于插入和更新而言可以统一起来，先看有没有，没有就直接设置，有就相加
                let value_in_map = hashmap.get(&operands[0].vecs[i][j]);
                let ans;
                match value_in_map {
                    Some(v) => ans = v.add(&value_weight),
                    None => ans = value_weight,
                }
                // 这里是覆盖插入，所以对于没有的值就是插入，对于有的值就是更新
                hashmap.insert(operands[0].vecs[i][j].clone(), ans);
                // 比已知的最大值大就替代
                maxlen = operands[0].vecs[i][j].gt(&maxlen).ite(&operands[0].vecs[i][j], &maxlen);
            }
            // 每次行遍历完成之后更新最大长度
            max_length = maxlen.gt(&max_length).ite(&maxlen, &max_length);
            hashmap_arr.push(hashmap);
        }
        // 这里直接把数组长度设置为min、max、求得的长度的最大值
        // 因为初始化之后就实现了大于最小值的部分用0填充
        let len_option = max_length.gt(&min).ite(&max_length.gt(&max).ite(&max_length, &max), &min).as_u64();
        #[allow(unused_assignments)]
        let mut result_len = 0;
        match len_option {
            Some(l) => result_len = l,
            None => result_len = 0,
        }
        let mut result: Vecs<Int> = Vecs::new([size0[0], (result_len + 1) as usize]);
        // 结果数组初始化
        for i in 0..size0[0] {
            for j in 0..((result_len + 1) as usize) {
                result.vecs[i][j] = zero(context, bit_width);
            }
        }
        for i in 0..size0[0] {
            for (key, value) in &hashmap_arr[i] {
                // key就是在数组中的位置，value就是对应的值，只不过bv得先转换一下
                // 比较里面的键和最大长度，比它大的会忽略
                let is_continue = key.gt(&max).ite(&one(context, bit_width), &zero(context, bit_width));
                if is_continue == one(context, bit_width) {
                    continue;
                }
                let key_option = key.as_u64();
                #[allow(unused_assignments)]
                let mut index = 0;
                match key_option {
                    Some(i) => index = i,
                    None => index= 0,
                }
                result.vecs[i][index as usize] = (*value).clone();
            }
        }
        return result
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfCountNonzero(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 遍历输入，找出非0的数并统计个数即可
        let size = operands[0].dims;
        let mut ans = zero(context, bit_width);
        for i in 0..size[0] {
            for j in 0..size[1] {
                // 为了方便计数，可以用z3判断是不是为0，然后用z3的方式相加
                let is_zero = operands[0].vecs[i][j]._eq(&zero(context, bit_width)).ite(&zero(context, bit_width),&one(context, bit_width));
                ans = ans.add(&is_zero);
            }
        }
        let mut result: Vecs<Int> = Vecs::new([1, 1]);
        result.vecs[0][0] = ans;
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
        3
    }

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfCumsum(operands[0], operands[1], operands[2])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 第一个是输入的数组
        // 第二个是判断算不算第一个，加入一个数组[a, b, c]，如果第二个参数是真，那么结果是[a, a + b, a + b + c]，否则就是[0, a, a + b]
        // 第三个是判断是正序还是倒序
        let size = operands[0].dims;
        let mut ans = zero(context, bit_width);
        let mut result: Vecs<Int> = Vecs::new(size);
        // 1为true，0为false
        let is_add_first = operands[1].vecs[0][0]._eq(&one(context, bit_width)).ite(&one(context, bit_width),&zero(context, bit_width));
        let is_reverse = operands[2].vecs[0][0]._eq(&one(context, bit_width)).ite(&one(context, bit_width),&zero(context, bit_width));
        if is_add_first == zero(context, bit_width) {
            for i in 0..size[0] {
                // 清零操作
                ans = zero(context, bit_width);
                for j in 0..size[1] {
                    result.vecs[i].push(ans.clone().add(&operands[0].vecs[i][j]));
                    ans = ans.clone().add(&operands[0].vecs[i][j]);
                }
            }
        } else {
            for i in 0..size[0] {
                result.vecs[i].push(zero(context, bit_width));
                for j in 0..size[1] - 1 {
                    result.vecs[i].push(ans.clone().add(&operands[0].vecs[i][j]));
                    ans = ans.clone().add(&operands[0].vecs[i][j]);
                }
            }
        }
        if is_reverse == one(context, bit_width) {
            for i in 0..size[0] {
                result.vecs[i].reverse();
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfMaximum(operands[0], operands[1])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 遍历比较求出最大的那个并放入结果中即可
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j].gt(&operands[1].vecs[i][j]).ite(&operands[0].vecs[i][j], &operands[1].vecs[i][j]));
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
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 遍历比较求出最小的那个并放入结果中即可
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let size_x_max = if size0[0] > size1[0] {
            size0[0]
        } else {
            size1[0]
        };
        let size_y_max = if size0[1] > size1[1] {
            size0[1]
        } else {
            size1[1]
        };
        // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
        // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
        // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
        // 能广播，第一种情况是两个维度上一定有一个是一样的
        if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
            // 横向维度一样，扩展纵向
            if size0[0] == size1[0] {
                // 第一个长度小于第二个
                if size0[1] < size1[1] {
                    for i in 0..size0[0] {
                        for j in size0[1]..size1[1] {
                            // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                        }
                    }
                } else {
                    for i in 0..size1[0] {
                        for j in size1[1]..size0[1] {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                        }
                    }
                }
            // 纵向维度一样，扩展横向
            } else if size0[1] == size1[1] {
                // 第一个长度小于第二个
                if size0[0] < size1[0] {
                    for i in size0[0]..size1[0] {
                        // 和上面的例子一样
                        operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                    }
                } else {
                    for i in size1[0]..size0[0] {
                        operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                    }
                }
            }
        // 虽不相同，但是其中有两个维度是1，也可以扩展
        } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
            if size0[1] == 1 {
                for i in 0..size0[0] {
                    // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                    for _j in 0..size1[1] - 1 {
                        operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                    }
                }
            }
            if size1[1] == 1 {
                for i in 0..size1[0] {
                    for _j in 0..size0[1] - 1 {
                        operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                    }
                }
            }
            if size0[0] == 1 {
                for _i in 0..size1[0] - 1 {
                    operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                }
            }
            if size1[0] == 1 {
                for _i in 0..size0[0] - 1 {
                    operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                }
            }
        }
        let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
        for i in 0..size_x_max {
            for j in 0..size_y_max {
                result.vecs[i].push(operands[0].vecs[i][j].lt(&operands[1].vecs[i][j]).ite(&operands[0].vecs[i][j], &operands[1].vecs[i][j]));
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfReverse(operands[0])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 倒序，就是把第size - index个放到index下标里
        let size = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(size);
        for i in 0..size[0] {
            for j in 0..size[1] {
                result.vecs[i].push(operands[0].vecs[i][size[1] - j - 1].clone());
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfSign(operands[0])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 确定每个数字的正负，正数1、负数-1、0为0
        let size = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(size);
        // 手动定义1、-1、0，只不过-1没有函数，得自己用方法设定
        let plus_one = one(context, bit_width);
        let zero = zero(context, bit_width);
        let minus_one = Int::from_i64(context, -1);
        for i in 0..size[0] {
            for j in 0..size[1] {
                // 先判断是否为负，是则返回-1，否则判断是否为正，是则返回1，否则不是正数也不是负数，则为0
                result.vecs[i].push(operands[0].vecs[i][j].lt(&zero).ite(&minus_one, &operands[0].vecs[i][j].gt(&zero).ite(&plus_one, &zero)));
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfSquare(operands[0])
    }

    fn make_expression<'a>(
        &self,
        _context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        _bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 遍历每个元素，返回每个元素的平方即可
        let size = operands[0].dims;
        let mut result: Vecs<Int> = Vecs::new(size);
        for i in 0..size[0] {
            for j in 0..size[1] {
                // 平方就是自己乘以自己
                result.vecs[i].push(operands[0].vecs[i][j].clone().mul(&operands[0].vecs[i][j]));
            }
        }
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

    fn make_operator(&self, _immediates: &Vec<Vecs<i64>>, operands: &[Id]) -> Operator {
        Operator::TfWhere(operands[0], operands[1], operands[2])
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        _immediates: &[Vecs<Int<'a>>],
        operands: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Vecs<Int<'a>> {
        // 根据输入的不同，方法不同。只有第一个是返回非0的位置，有后面输入，如果第一个是true则返回第二个，否则是第三个
        // rust能重载，但好像比较麻烦，所以目前先按照如果第二个和第三个输入为0则为未输入
        let mut flag = false;
        let size0 = operands[0].dims;
        let size1 = operands[1].dims;
        let size2 = operands[2].dims;
        for i in 0..size1[0] {
            for j in 0..size1[1] {
                if operands[1].vecs[i][j] != zero(context, bit_width) {
                    flag = true;
                }
            }
        }
        for i in 0..size2[0] {
            for j in 0..size2[1] {
                if operands[2].vecs[i][j] != zero(context, bit_width) {
                    flag = true;
                }
            }
        }
        // 没有输入第二个和第三个数，那么就是返回非0的位置
        if flag == false {
            // 记录下个数，作为结果的维度用
            let mut count = 0;
            // 要求建立结果的时候就要确定维度，可是一开始并不知道有多长，所以就先临时存储，最后取出即可
            let mut index_arr: Vec<Vec<Int>> = Vec::new();
            for i in 0..size0[0] {
                for j in 0..size0[1] {
                    if operands[0].vecs[i][j] != zero(context, bit_width) {
                        // 要注意结果按照bitvec存储
                        index_arr.push([Int::from_i64(context, i as i64), Int::from_i64(context, j as i64)].to_vec());
                        count += 1;
                    }
                }
            }
            // 结果数组的行数就是非0的个数，列数就是几个维度
            let mut result: Vecs<Int> = Vecs::new([count as usize, 2]);
            for i in 0..index_arr.len() {
                result.vecs.push(index_arr[i].clone());
            }
            return result;
        } else {
            // 其余情况就是如果为true则返回第二个参数，为false则返回第三个参数
            // 用最笨的办法求最大长度，rust你为什么没有min或者max函数！！！！！！！！！！
            let size_x_max = if size0[0] > size1[0] && size0[0] > size2[0] {
                size0[0]
            } else if size1[0] > size0[0] && size1[0] > size2[0] {
                size1[0]
            } else {
                size2[0]
            };
            let size_y_max = if size0[1] > size1[1] && size0[1] > size2[1] {
                size0[1]
            } else if size1[1] > size0[1] && size1[1] > size2[1] {
                size1[1]
            } else {
                size2[1]
            };
            // 三个数组广播，那就两两广播
            // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
            // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
            // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
            // 能广播，第一种情况是两个维度上一定有一个是一样的
            if size0[1] == size1[1] && size0[0] != size1[0] || size0[0] == size1[0] && size0[1] != size1[1] {
                // 横向维度一样，扩展纵向
                if size0[0] == size1[0] {
                    // 第一个长度小于第二个
                    if size0[1] < size1[1] {
                        for i in 0..size0[0] {
                            for j in size0[1]..size1[1] {
                                // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                                operands[0].clone().vecs[i].push(operands[0].vecs[i][j % size0[1]].clone());
                            }
                        }
                    } else {
                        for i in 0..size1[0] {
                            for j in size1[1]..size0[1] {
                                operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                            }
                        }
                    }
                // 纵向维度一样，扩展横向
                } else if size0[1] == size1[1] {
                    // 第一个长度小于第二个
                    if size0[0] < size1[0] {
                        for i in size0[0]..size1[0] {
                            // 和上面的例子一样
                            operands[0].clone().vecs.push(operands[0].vecs[i % size0[0]].clone());
                        }
                    } else {
                        for i in size1[0]..size0[0] {
                            operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                        }
                    }
                }
            // 虽不相同，但是其中有两个维度是1，也可以扩展
            } else if size0[0] != size1[0] && size0[1] != size1[1] && (size0[0] == 1 || size0[1] == 1 || size1[0] == 1 || size1[1] == 1) {
                if size0[1] == 1 {
                    for i in 0..size0[0] {
                        // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                        for _j in 0..size1[1] - 1 {
                            operands[0].clone().vecs[i].push(operands[0].vecs[i][0].clone());
                        }
                    }
                }
                if size1[1] == 1 {
                    for i in 0..size1[0] {
                        for _j in 0..size0[1] - 1 {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                        }
                    }
                }
                if size0[0] == 1 {
                    for _i in 0..size1[0] - 1 {
                        operands[0].clone().vecs.push(operands[0].vecs[0].clone());
                    }
                }
                if size1[0] == 1 {
                    for _i in 0..size0[0] - 1 {
                        operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                    }
                }
            }
            // 广播规则：维度低的向量有1，则扩充，也就是标量会扩充到和向量一样的维度
            // 尾部尺寸一致的，按照行进行扩充，如4行3列和1行3列的数组，都是3列，所以1行3列的数组会扩充到4行3列
            // 二者结合，比如3行1列和1行3列的数组，因为有1列和3列，所以都会扩充到3行3列
            // 能广播，第一种情况是两个维度上一定有一个是一样的
            if size2[1] == size1[1] && size2[0] != size1[0] || size2[0] == size1[0] && size2[1] != size1[1] {
                // 横向维度一样，扩展纵向
                if size2[0] == size1[0] {
                    // 第一个长度小于第二个
                    if size2[1] < size1[1] {
                        for i in 0..size2[0] {
                            for j in size2[1]..size1[1] {
                                // 举个例子，一个长度为2的数组扩展到长度为5，那么要从下标为2开始，下标为5（不含）中止，每次要放入的数就是长度为2数组下标为0、1、0、1的数
                                operands[2].clone().vecs[i].push(operands[2].vecs[i][j % size2[1]].clone());
                            }
                        }
                    } else {
                        for i in 0..size1[0] {
                            for j in size1[1]..size2[1] {
                                operands[1].clone().vecs[i].push(operands[1].vecs[i][j % size1[1]].clone());
                            }
                        }
                    }
                // 纵向维度一样，扩展横向
                } else if size2[1] == size1[1] {
                    // 第一个长度小于第二个
                    if size2[0] < size1[0] {
                        for i in size2[0]..size1[0] {
                            // 和上面的例子一样
                            operands[2].clone().vecs.push(operands[2].vecs[i % size2[0]].clone());
                        }
                    } else {
                        for i in size1[0]..size2[0] {
                            operands[1].clone().vecs.push(operands[1].vecs[i % size1[0]].clone());
                        }
                    }
                }
            // 虽不相同，但是其中有两个维度是1，也可以扩展
            } else if size2[0] != size1[0] && size2[1] != size1[1] && (size2[0] == 1 || size2[1] == 1 || size1[0] == 1 || size1[1] == 1) {
                if size2[1] == 1 {
                    for i in 0..size2[0] {
                        // 既然一个为1，另一个就不是1，所以扩展的最终长度是另一个的长度
                        for _j in 0..size1[1] - 1 {
                            operands[2].clone().vecs[i].push(operands[2].vecs[i][0].clone());
                        }
                    }
                }
                if size1[1] == 1 {
                    for i in 0..size1[0] {
                        for _j in 0..size2[1] - 1 {
                            operands[1].clone().vecs[i].push(operands[1].vecs[i][0].clone());
                        }
                    }
                }
                if size2[0] == 1 {
                    for _i in 0..size1[0] - 1 {
                        operands[2].clone().vecs.push(operands[2].vecs[0].clone());
                    }
                }
                if size1[0] == 1 {
                    for _i in 0..size2[0] - 1 {
                        operands[1].clone().vecs.push(operands[1].vecs[0].clone());
                    }
                }
            }
            let mut result: Vecs<Int> = Vecs::new([size_x_max, size_y_max]);
            for i in 0..size_x_max {
                for j in 0..size_y_max {
                    if operands[0].vecs[i][j] == zero(context, bit_width) {
                        result.vecs[i][j] = operands[1].vecs[i][j].clone();
                    } else {
                        result.vecs[i][j] = operands[2].vecs[i][j].clone();
                    }
                }
            }
            return result;
        }
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
            Operator::Const(c) => {
                let $c = Const(*c);
                $body
            }
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
            Operator::TfArgMax(_) => {
                let $c = TfArgMax;
                $body
            }
            Operator::TfArgMin(_) => {
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
            Operator::TfCumsum(_, _, _) => {
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