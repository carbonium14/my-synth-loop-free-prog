use crate::{Id, Instruction, Operator, Program, Vecs};

const DIMSIZE : [usize ; 2] = [4,10];

#[derive(Debug)]
pub struct ProgramBuilder {
    program: Program,
}

impl ProgramBuilder {
    pub fn new() -> ProgramBuilder {
        ProgramBuilder {
            program: Program {
                instructions: vec![],
                inputs: vec![]
            },
        }
    }

    pub fn finish(self) -> Program {
        self.program
    }

    fn next_id(&self) -> Id {
        Id(self.program.instructions.len() as u32)
    }

    pub fn var(&mut self, input : Vec<Vec<i64>>) -> Id {
        /*assert!(
            self.program
                .instructions
                .iter()
                .all(|inst| inst.operator == Operator::Var) ,
            "All `var`s must be at the start of the program"
        );*/

        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::Var,
        });

        //println!("{:?}", input);
        //将输入存入program中
        let mut dims : [usize; 2] = [0, 0];
        if input.len() == 0 {
            //to_asignment()函数调用
            let input_vecs : Vecs<Vec<Vec<i64>>> = Vecs::new(dims, input);
            self.program.inputs.push(input_vecs);
            
        } else {
            // dims[0] = input.len();
            // dims[1] = input[0].len();
            // let input_vecs : Vecs<Vec<Vec<i64>>> = Vecs::new(dims, input);
            // self.program.inputs.push(input_vecs);

            // 对输入数组进行填充，多余的部分直接使用0填充成[DIMSSIZE0][DIMSSIZE1]的数组
            // [初始值 ； 出现次数]
            dims[0] = input.len();
            dims[1] = input[0].len();

            let mut arr : Vec<Vec<i64>> = Vec::new();
            for _i in 0 .. DIMSIZE[0] {
                let mut temp : Vec<i64>= Vec::new();
                for _j in 0 .. DIMSIZE[1] {
                    temp.push(0);
                }
                arr.push(temp);
            }
            for i in 0 .. input.len() {
                for j in 0 .. input[0].len() {
                    arr[i][j] = input[i][j];
                }
            }
            let input_vecs : Vecs<Vec<Vec<i64>>> = Vecs::new(dims, arr);

            //println!("{:?}", input_vecs);
            self.program.inputs.push(input_vecs);

        }

        result
        
        
    }

    /*pub fn vecs(&mut self, c: u64) -> Id {
        /*assert!(
            self.program
                .instructions
                .iter()
                .all(|inst| inst.operator == Operator::Var) ,
            "All `var`s must be at the start of the program"
        );*/

        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::Vecs(c),
        });
        result
    }*/

    // pub fn const_(&mut self, c: Vec<Vec<i64>>) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::Const(c),
    //     });
    //     result
    // }

    pub fn tf_abs(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfAbs(a),
        });
        result
    }

    // pub fn tf_add(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfAdd(a, b),
    //     });
    //     result
    // }

    // pub fn tf_mul(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfMul(a, b),
    //     });
    //     result
    // }

    // pub fn tf_div(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfDiv(a, b),
    //     });
    //     result
    // }

    // pub fn tf_argmax(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfArgMax(a),
    //     });
    //     result
    // }

    // pub fn tf_argmin(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfArgMin(a),
    //     });
    //     result
    // }

    pub fn tf_boolean_mask(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfBooleanMask(a, b),
        });
        result
    }

    // pub fn tf_cast(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfCast(a),
    //     });
    //     result
    // }

    // pub fn tf_clip_by_value(&mut self, a: Id, b: Id, c: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfClipByValue(a, b, c),
    //     });
    //     result
    // }

    // pub fn tf_equal(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfEqual(a, b),
    //     });
    //     result
    // }

    // pub fn tf_eye(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfEye(a, b),
    //     });
    //     result
    // }

    // pub fn tf_fill(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfFill(a, b),
    //     });
    //     result
    // }

    // pub fn tf_greater(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfGreater(a, b),
    //     });
    //     result
    // }

    // pub fn tf_greater_equal(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfGreaterEqual(a, b),
    //     });
    //     result
    // }

    // pub fn tf_not_equal(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfNotEqual(a, b),
    //     });
    //     result
    // }

    // pub fn tf_negative(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfNegative(a),
    //     });
    //     result
    // }

    // pub fn tf_reciprocal(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfReciprocal(a),
    //     });
    //     result
    // }

    // pub fn tf_bincount(&mut self, a: Id, b: Id, c: Id, d: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfBincount(a, b, c, d),
    //     });
    //     result
    // }



    // pub fn tf_count_nonzero(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfCountNonzero(a),
    //     });
    //     result
    // }

    // pub fn tf_cumsum(&mut self, a: Id, b: Id, c: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfCumsum(a, b, c),
    //     });
    //     result
    // }

    // pub fn tf_maximum(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfMaximum(a, b),
    //     });
    //     result
    // }

    // pub fn tf_minimum(&mut self, a: Id, b: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfMinimum(a, b),
    //     });
    //     result
    // }

    // pub fn tf_reverse(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfReverse(a),
    //     });
    //     result
    // }

    // pub fn tf_sign(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfSign(a),
    //     });
    //     result
    // }

    // pub fn tf_square(&mut self, a: Id) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::TfSquare(a),
    //     });
    //     result
    // }

}
