use crate::{Id, Instruction, Operator, Program };

const _DIMSIZE : [usize ; 2] = [4,10];

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
        self.program.inputs.push(input);
        result
    }

    // pub fn const_(&mut self, c: Vec<Vec<i64>>) -> Id {
    //     let result = self.next_id();
    //     self.program.instructions.push(Instruction {
    //         result,
    //         operator: Operator::Const(c),
    //     });
    //     result
    // }

    pub fn tf_add(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfAdd(a, b),
        });
        result
    }

    pub fn tf_argmax(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfArgmax(a, b),
        });
        result
    }

    pub fn tf_cast(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfCast(a),
        });
        result
    }

    pub fn tf_constant(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfConstant(a),
        });
        result
    }

    pub fn tf_divide(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfDivide(a, b),
        });
        result
    }

    pub fn tf_equal(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfEqual(a, b),
        });
        result
    }

    pub fn tf_greater(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfGreater(a, b),
        });
        result
    }

    pub fn tf_multiply(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfMultiply(a, b),
        });
        result
    }

    pub fn tf_square(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfSquare(a),
        });
        result
    }

    pub fn tf_subtract(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfSubtract(a, b),
        });
        result
    }


    pub fn tf_eye(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfEye(a, b),
        });
        result
    }

    pub fn tf_fill(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfFill(a, b),
        });
        result
    }

    pub fn tf_maximum(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfMaximum(a, b),
        });
        result
    }

    pub fn tf_minimum(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfMinimum(a, b),
        });
        result
    }

    pub fn tf_not_equal(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfNotEqual(a, b),
        });
        result
    }

    pub fn tf_ones(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfOnes(a),
        });
        result
    }

    pub fn tf_zeros(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfZeros(a),
        });
        result
    }

}
