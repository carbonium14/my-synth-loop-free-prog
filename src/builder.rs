use crate::{Id, Instruction, Operator, Program};

#[derive(Debug)]
pub struct ProgramBuilder {
    program: Program,
}

impl ProgramBuilder {
    pub fn new() -> ProgramBuilder {
        ProgramBuilder {
            program: Program {
                instructions: vec![],
            },
        }
    }

    pub fn finish(self) -> Program {
        self.program
    }

    fn next_id(&self) -> Id {
        Id(self.program.instructions.len() as u32)
    }

    pub fn var(&mut self) -> Id {
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

    pub fn const_(&mut self, c: Vec<u64>) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::Const(c),
        });
        result
    }

    pub fn tf_abs(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfAbs(a),
        });
        result
    }

    pub fn tf_add(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfAdd(a, b),
        });
        result
    }

    pub fn tf_mul(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfMul(a, b),
        });
        result
    }

    pub fn tf_div(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfDiv(a, b),
        });
        result
    }

    pub fn tf_argmax(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfArgMax(a),
        });
        result
    }

    pub fn tf_argmin(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfArgMin(a),
        });
        result
    }

    pub fn tf_boolean_mask(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfBooleanMask(a, b),
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

    pub fn tf_clip_by_value(&mut self, a: Id, b: Id, c: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfClipByValue(a, b, c),
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

    pub fn tf_fill(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfFill(a, b),
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

    pub fn tf_greater_equal(&mut self, a: Id, b: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfGreaterEqual(a, b),
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

    pub fn tf_negative(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfNegative(a),
        });
        result
    }

    pub fn tf_reciprocal(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfReciprocal(a),
        });
        result
    }

    pub fn tf_count_nonzero(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfCountNonzero(a),
        });
        result
    }

    pub fn tf_cumsum(&mut self, a: Id, b: Id, c: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfCumsum(a, b, c),
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

    pub fn tf_reverse(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfReverse(a),
        });
        result
    }

    pub fn tf_sign(&mut self, a: Id) -> Id {
        let result = self.next_id();
        self.program.instructions.push(Instruction {
            result,
            operator: Operator::TfSign(a),
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

}
