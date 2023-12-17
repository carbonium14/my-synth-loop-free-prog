#![deny(missing_debug_implementations)]

#[cfg(feature = "log")]
#[macro_use]
extern crate log;

#[cfg(not(feature = "log"))]
#[macro_use]
mod fake_logging;

mod builder;
pub mod component;
mod operator;

pub use builder::ProgramBuilder;
pub use component::Component;
pub use operator::Operator;

use std::collections::{HashMap, HashSet};
use std::fmt::{self, Display};
use std::iter::FromIterator;
use std::ops::Range;
use std::time;
use z3::ast::{Ast, Bool, Int};

const _FULL_BIT_WIDTH: u32 = 32;

const DIMS : [usize; 2] = [4, 10];

// enum Type<'a> {
//     intVar(Int<'a>),
//     floatVar(Float<'a>)
// }
//可以用match判断是哪一种type


fn and<'a, 'b>(context: &'a z3::Context, exprs: impl IntoIterator<Item = &'b Bool<'a>>) -> Bool<'a>
where
    'a: 'b,
{
    let exprs: Vec<&_> = exprs.into_iter().collect();

    //Bool::from_bool(context, true).and(&exprs)
    return z3::ast::Bool::<'a>::and(&context, &[&Bool::from_bool(context, true), 
    &(z3::ast::Bool::<'a>::and(&context, &exprs))]);
}

fn _or<'a, 'b>(context: &'a z3::Context, exprs: impl IntoIterator<Item = &'b Bool<'a>>) -> Bool<'a>
where
    'a: 'b,
{
    let exprs: Vec<&_> = exprs.into_iter().collect();

    //Bool::from_bool(context, false).or(&exprs)
    return z3::ast::Bool::<'a>::or(&context, &[&Bool::from_bool(context, false), 
    &(z3::ast::Bool::<'a>::or(&context, &exprs))]);
}

//对于多维数组的扩散，我们可以使用一个纬度数组，然后动态创建一个嵌套了这么多层的vecs
//TODO : 但是目前找不到一个方法将vecs的类型泛型表示出来，对于几纬就只能固定为几个Vec<Vec<...>>
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vecs<T>{
    dims : [T ; 2],
    vecs : Vec<Vec<T>>,
}

impl<T> Vecs<T>{
    pub fn new(dims: [T ; 2]) -> Self {
        let _sz = dims.len();
        let mut vecs : Vec<Vec<T>> = Vec::new();
        for _ in 0 .. DIMS[0] {
            let temp : Vec<T> = Vec::new();
            vecs.push(temp);
        }
        return Vecs{dims, vecs};
    }   
}

//TODO: 动态维度，不过目前只能实现二维
fn fresh_immediate(context: &z3::Context, _bit_width: u32, dims:[usize; 2] ) ->  Vecs<Int<'_>> {
    let mut result: Vecs<Int<'_>> = Vecs::new([Int::from_i64(&context, dims[0] as i64), Int::from_i64(&context, dims[1] as i64)]);
    let x = dims[0];
    let y = dims[1];
    for i in 0 .. x {
        for _j in 0 .. y {
            result.vecs[i].push(Int::fresh_const(context, "immediate"));
        }
    }
    return result;
}

//TODO: 动态维度，不过目前只能实现二维
fn fresh_param(context: &z3::Context, _bit_width: u32, dims: [usize ; 2]) ->  Vecs<Int<'_>> {
    let mut result = Vecs::new([Int::from_i64(&context, dims[0] as i64), Int::from_i64(&context, dims[1] as i64)]);
    let x = dims[0];
    let y = dims[1];
    for i in 0 .. x {
        for _j in 0 .. y {
            result.vecs[i].push(Int::fresh_const(context, "param"));
        }
    }
    return result;
}

//TODO: 动态维度，不过目前只能实现二维
fn fresh_result(context: &z3::Context, _bit_width: u32, dims: [usize ; 2]) ->  Vecs<Int<'_>> {
    let mut result = Vecs::new([Int::from_i64(&context, dims[0] as i64), Int::from_i64(&context, dims[1] as i64)]);
    let x = dims[0];
    let y = dims[1];
    for i in 0 .. x {
        for _j in 0 .. y {
            result.vecs[i].push(Int::fresh_const(context, "result"));
        }
    }
    return result;
}

//TODO: 动态维度，不过目前只能实现二维
fn _fresh_input(context: &z3::Context, _bit_width: u32, dims: [usize ; 2]) ->  Vecs<Int<'_>> {
    let mut result = Vecs::new([Int::from_i64(&context, dims[0] as i64), Int::from_i64(&context, dims[1] as i64)]);
    let x = dims[0];
    let y = dims[1];
    for i in 0 .. x {
        for _j in 0 .. y {
            result.vecs[i].push(Int::fresh_const(context, "input"));
        }
    }
    return result;
}

//TODO: 动态维度，不过目前只能实现二维
fn fresh_output(context: &z3::Context, _bit_width: u32, dims: [usize ; 2]) ->  Vecs<Int<'_>> {
    let mut result = Vecs::new([Int::from_i64(&context, dims[0] as i64), Int::from_i64(&context, dims[1] as i64)]);
    let x = dims[0];
    let y = dims[1];
    for i in 0 .. x {
        for _j in 0 .. y {
            result.vecs[i].push(Int::fresh_const(context, "output"));
        }
    }
    return result;
}

fn eval_bitvec(model: &z3::Model, bv: &Int) -> i64 {
    model.eval(bv)
    .unwrap()
    .as_i64()
    .unwrap()
}

// fn as_f64(model: &z3::Model, fval: &Float) ->f64 {
//     let x = model.eval(fval).unwrap();

   
//     if Z3_get_numeral_double
    
//     (self.ctx.z3_ctx, self.z3_ast, &mut tmp) {
//             Some(tmp)
//         } else {
//             None
//         }
// }

//将bitvec转化为i64类型
fn eval_bitvecs<'a, I>(model: &'a z3::Model, bvs:I) -> Vec<Vecs<i64>>
where
    I: IntoIterator<Item = &'a Vecs< Int<'a>>>,
{
    let result : Vec<Vecs<i64>> = Vec::new();
    for v in bvs.into_iter() {
        let size_x = v.dims[0].as_i64().unwrap();
        let size_y = v.dims[1].as_i64().unwrap();
        let mut temp : Vecs<i64> = Vecs::new([size_x, size_y]);
        for i in 0 .. size_x as usize {
            for j in 0 .. size_y as usize {
                temp.vecs[i].push(eval_bitvec(model, &v.vecs[i][j]));
            }
        }

        // for k in 1..v.len()+1 {
        //     temp.push(eval_bitvec(model, &v[k-1]));
        // }
        // result.push(temp);

    }
    return result;
}


/*fn eval_bitvecs<'a, I>(model: &'a z3::Model, bvs: I) -> Vec<u64>
where
    I: IntoIterator<Item = &'a Vec<BitVec<'a>>>,
{
    bvs.into_iter()
        .map(move |bv| eval_bitvec(model, &bv[0]))
        .collect()
}*/

fn eval_line(model: &z3::Model, line: &Line) -> u32 {
    eval_bitvec(model, line) as u32
}

fn eval_lines<'a, I>(model: &'a z3::Model, lines: I) -> Vec<u32>
where
    I: IntoIterator<Item = &'a Line<'a>>,
{
    lines
        .into_iter()
        .map(move |l| eval_line(model, l))
        .collect()
}

#[derive(Debug)]
pub enum Error {
    NoComponents,
    SynthesisUnsatisfiable,
    SynthesisUnknown,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u32);

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut x = self.0;
        loop {
            let y = x % 26;
            x = x / 26;

            s.insert(0, ('a' as u32 + y) as u8 as char);

            if x == 0 {
                break;
            }

            x -= 1;
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Instruction {
    result: Id,
    operator: Operator,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ← {}", self.result, self.operator)
    }
}

pub trait Specification: fmt::Debug {
    fn arity(&self) -> usize;

    //获取输入
    fn inputs(&self) -> Vec<&Vec<Vec<i64>>>;

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        // inputs: &Vec<Vec<BitVec<'a>>>,
        // output: &Vec<BitVec<'a>>,
        inputs: &Vec<Vecs<Int<'a>>>,
        output: &Vecs<Int<'a>>,
        bit_width: u32,
    ) -> Bool<'a>;
}

/// A collection of components.
///
/// Multiple copies of a particular component may exist in the library, allowing
/// synthesis to use it multiple times.
#[derive(Debug)]
pub struct Library {
    pub components: Vec<Box<dyn Component>>,
}

impl Library {
    /// Create a library of components that is roughly equivalent to the Brahma
    /// standard library.
    pub fn brahma_std() -> Self {
        Library {
            // 7.3 Choice of Multi-set of Base Components:
            //
            // > The standard library included 12 components, one each for
            // > performing standard operations, such as bitwise-and,
            // > bitwise-or, bitwise-not, add-one, bitwise-xor, shift-right,
            // > comparison, add, and subtract operations."
            //
            // They don't actually spell out exactly what's included, but here
            // are the set of components used in the benchmark problems they say
            // are solved with just the standard components:
            //
            //  1. add
            //  2. and
            //  3. neg
            //  4. not
            //  5. or
            //  6. shr (signed)
            //  7. shr (logical)
            //  8. sub
            //  9. uge
            // 10. ugt
            // 11. ule
            // 12. xor
            //
            // Note that they only use "bvshr" which doesn't specify if the
            // right shift is signed or logical, but `p13` uses two right
            // shifts, and for it to be correct the first has to be signed and
            // the second has to be logical. It was really annoying to figure
            // that out!
            //
            // Finally, it isn't 100% clear to me if they synthesized the
            // various constants that appear in their solutions, or if they
            // provided them as components. By my reading, it sort of seems like
            // they did a mix. So we leave constants out of this library, and
            // kick that problem down the road to callers.
            components: vec![
                // // 1.
                // component::add(),
                // // 2.
                // component::and(),
                // // 3. neg(x) = 0 - x
                // component::const_(Some(0)),
                // component::sub(),
                // // 4. not(a) = xor a, MAX
                // component::const_(Some(std::u64::MAX)),
                // component::xor(),
                // // 5.
                // component::or(),
                // // 6.
                // component::shr_s(),
                // // 7.
                // component::shr_u(),
                // // 8.
                // component::sub(),
                // // 9.
                // component::ge_u(),
                // // 10.
                // component::gt_u(),
                // // 11. ule
                // component::le_u(),
                // // 12.
                // component::xor(),
                component::tf_add(),
                component::tf_cast(),
                component::tf_constant(),
                component::tf_equal(),
                component::tf_multiply(),
                component::tf_square(),
                component::tf_subtract(),
            ],
        }
    }
}

type Line<'a> = Int<'a>;

fn line_lt<'a>(lhs: &Line<'a>, rhs: &Line<'a>) -> Bool<'a> {
    lhs.lt(rhs)
}

fn line_le<'a>(lhs: &Line<'a>, rhs: &Line<'a>) -> Bool<'a> {
    lhs.le(rhs)
}

#[derive(Debug)]
struct LocationVars<'a> {
    inputs: Vec<Line<'a>>,
    params: Vec<Line<'a>>,
    results: Vec<Line<'a>>,
    output: Line<'a>,
    line_bit_width: u32,
}

impl<'a> LocationVars<'a> {
    fn new(context: &'a z3::Context, library: &Library, num_inputs: usize) -> Self {
        let max_line = num_inputs as u32
            + library.components.len() as u32
            + library
                .components
                .iter()
                .map(|c| c.operand_arity() as u32)
                .sum::<u32>();
        let max_pow_2 = (max_line + 1).next_power_of_two();
        // `2^n` are of the form `0b10..0` and `trailing_zeros(2^n)` is `n`. For
        // example, `4 = 0b100` and `trailing_zeros(4) = 2`. Since we already
        // added one before we took the next power of two, we only need as many
        // bits as there are trailing zeros to represent every possible line
        // number.
        let line_bit_width = max_pow_2.trailing_zeros();

        let inputs = (0..num_inputs)
            .map(|_| Self::fresh_line(context, "input_location", line_bit_width))
            .collect();
        let params = library
            .components
            .iter()
            .flat_map(|c| {
                (0..c.operand_arity())
                    .map(|_| Self::fresh_line(context, "param_location", line_bit_width))
            })
            .collect();
        let results = library
            .components
            .iter()
            .map(|_| Self::fresh_line(context, "result_location", line_bit_width))
            .collect();
        let output = Self::fresh_line(context, "output_line", line_bit_width);
        LocationVars {
            inputs,
            params,
            results,
            output,
            line_bit_width,
        }
    }

    fn fresh_line(context: &'a z3::Context, name: &str, _line_bit_width: u32) -> Line<'a> {
        Int::fresh_const(context, name)
    }

    fn line_from_u32(&self, context: &'a z3::Context, line: u32) -> Line<'a> {
        assert!(line < (1 << self.line_bit_width));
        Int::from_i64(context, line as i64)
    }

    fn inputs_range(&self) -> Range<u32> {
        0..self.inputs.len() as u32
    }

    fn params_range(&self) -> Range<u32> {
        let start = self.inputs.len() as u32;
        let end = start + self.params.len() as u32;
        start..end
    }

    fn results_range(&self) -> Range<u32> {
        let start = self.inputs.len() as u32 + self.params.len() as u32;
        let end = start + self.results.len() as u32;
        start..end
    }

    fn output_range(&self) -> Range<u32> {
        let start = self.inputs.len() as u32 + self.params.len() as u32 + self.results.len() as u32;
        let end = start + 1;
        start..end
    }

    fn invalid_connections(&self, library: &Library) -> HashSet<(u32, u32)> {
        let mut invalid_connections = HashSet::new();

        // We will never assign the output directly to an input.
        for a in self.inputs_range() {
            for b in self.output_range() {
                invalid_connections.insert((a, b));
            }
        }

        // We never assign an input's location to another input's location.
        for (i, a) in self.inputs_range().enumerate() {
            for b in self.inputs_range().skip(i as usize) {
                invalid_connections.insert((a, b));
            }
        }

        // Similarly, a well-formed program will never assign a param's location
        // as another param; it should only be one of the original inputs or the
        // result of another component.
        for (i, p) in self.params_range().enumerate() {
            for q in self.params_range().skip(i as usize) {
                invalid_connections.insert((p, q));
            }
        }

        // Finally, a well-formed will never have a component with its own
        // result as a parameter.
        let params = &mut self.params_range();
        for (r, c) in self.results_range().zip(&library.components) {
            for p in params.take(c.operand_arity()) {
                invalid_connections.insert((r, p));
            }
        }

        invalid_connections
    }

    /// 5.1 Encoding Well-formed Programs
    fn well_formed_program(
        &self,
        context: &'a z3::Context,
        library: &Library,
        invalid_connections: &mut HashSet<(u32, u32)>,
    ) -> Bool<'a> {
        let mut wfp = Vec::with_capacity(
            // Acyclic and consistent.
            2
                // Assignment of inputs.
                + self.inputs.len()
                // Lower and upper bounds on params.
                + self.params.len() * 2
                // Lower and upper bounds on results.
                + self.results.len() * 2,
        );

        wfp.push(self.consistent(context, invalid_connections));
        wfp.push(self.acyclic(context, library));

        let i_len = self.line_from_u32(context, self.inputs.len() as u32);
        let m = self.line_from_u32(context, (self.results.len() + self.inputs.len()) as u32);
        let zero = self.line_from_u32(context, 0);

        for (i, l) in self.inputs.iter().enumerate() {
            let i = self.line_from_u32(context, i as u32);
            wfp.push(l._eq(&i));
        }

        for l in &self.params {
            // 0 <= l
            wfp.push(line_le(&zero, l));
            // l < M
            wfp.push(line_lt(l, &m));
        }

        for l in &self.results {
            // |i| <= l
            wfp.push(line_le(&i_len, l));
            // l < m
            wfp.push(line_lt(l, &m));
        }

        and(context, &wfp)
    }

    fn consistent(
        &self,
        context: &'a z3::Context,
        invalid_connections: &mut HashSet<(u32, u32)>,
    ) -> Bool<'a> {
        let mut cons = vec![];
        for (i, (i_x, x)) in self.results_range().zip(&self.results).enumerate() {
            for (i_y, y) in self.results_range().zip(&self.results).skip(i + 1) {
                invalid_connections.insert((i_x, i_y));
                cons.push(x._eq(y).not());
            }
        }
        and(context, &cons)
    }

    fn acyclic(&self, context: &'a z3::Context, library: &Library) -> Bool<'a> {
        let mut acycs = vec![];
        let mut params = self.params.iter();
        let mut results = self.results.iter();

        for c in &library.components {
            let result_location = results.next().unwrap();
            for _ in 0..c.operand_arity() {
                let param_location = params.next().unwrap();
                acycs.push(line_lt(param_location, result_location));
            }
        }

        and(context, &acycs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Assignments {
    immediates: Vec<Vecs<i64>>,
    // The line in the program where the i^th input is defined (for all inputs
    // of all components).
    params: Vec<u32>,
    // The line in the program where the i^th component is located (and
    // therefore the i^th output is defined)..
    results: Vec<u32>,
    // The line in the program where the final output is defined.
    output: u32,
}

impl Assignments {
    fn to_program(&self, num_inputs: usize, library: &Library) -> Program {
        let mut b = ProgramBuilder::new();
        for _ in 0..num_inputs {
            //这里没必要传数组参数了，直接给值就行
            b.var(Vec::new());
        }
        let mut program = b.finish();

        let mut immediates = self.immediates.iter().cloned();
        let mut params = self.params.iter().cloned().map(Id);

        program
            .instructions
            .extend(self.results.iter().zip(&library.components).map(|(&n, c)| {
                let imm_arity = c.immediate_arity();
                let immediates: Vec<_> = immediates.by_ref().take(imm_arity).collect();

                let op_arity = c.operand_arity();
                let operands: Vec<_> = params.by_ref().take(op_arity).collect();
                debug_assert!(operands.iter().all(|op| op.0 < n));

                let operator = c.make_operator(&immediates, &operands);
                let result = Id(n);
                Instruction { result, operator }
            }));

        program.instructions.sort_unstable_by_key(|i| i.result.0);
        program.instructions.truncate(self.output as usize + 1);
        program
    }
}

#[derive(Debug)]
pub struct Program {
    instructions: Vec<Instruction>,
    inputs: Vec<Vec<Vec<i64>>>
}

impl Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &self.instructions {
            write!(f, "{}\n", i)?;
        }
        Ok(())
    }
}

enum _Verification {
    WorksForAllInputs,
    Counterexample(Vec<Vecs<u64>>),
}

#[derive(Debug, Clone)]
enum Timeout {
    Duration(time::Duration),
    Instant(time::Instant),
}

#[derive(Debug)]
pub struct Synthesizer<'a> {
    context: &'a z3::Context,
    library: &'a Library,
    spec: &'a dyn Specification,
    locations: LocationVars<'a>,
    well_formed_program: Bool<'a>,
    invalid_connections: HashSet<(u32, u32)>,
    not_invalid_assignments: Bool<'a>,
    should_synthesize_minimal_programs: bool,
    timeout: Option<Timeout>,
}

impl<'a> Synthesizer<'a> {
    pub fn new(
        context: &'a z3::Context,
        library: &'a Library,
        spec: &'a dyn Specification,
    ) -> Result<Self> {
        if library.components.is_empty() {
            return Err(Error::NoComponents);
        }

        let locations = LocationVars::new(context, library, spec.arity());
        let mut invalid_connections = locations.invalid_connections(library);
        let well_formed_program =
            locations.well_formed_program(context, library, &mut invalid_connections);
        let not_invalid_assignments = Bool::from_bool(context, true);
        Ok(Synthesizer {
            context,
            library,
            spec,
            locations,
            well_formed_program,
            invalid_connections,
            not_invalid_assignments,
            should_synthesize_minimal_programs: false,
            timeout: None,
        })
    }

    /// Configure whether we should synthesize the minimal-length program that
    /// satisfies the specification.
    ///
    /// This produces the smallest possible program, but it tends to take
    /// more time.
    pub fn should_synthesize_minimal_programs(&mut self, should: bool) -> &mut Self {
        self.should_synthesize_minimal_programs = should;
        self
    }

    /// Configure the timeout.
    ///
    /// No timeout means that we will keep going forever if necessary. Providing
    /// a number of milliseconds means we will have a soft maximum runtime of of
    /// that many milliseconds before giving up.
    pub fn set_timeout(&mut self, milliseconds: Option<u32>) -> &mut Self {
        self.timeout =
            milliseconds.map(|ms| Timeout::Duration(time::Duration::from_millis(ms as u64)));
        self
    }

    fn solver(&mut self) -> z3::Solver<'a> {
        let solver = z3::Solver::new(self.context);

        if let Some(timeout) = self.timeout.clone() {
            let millis = match timeout {
                Timeout::Duration(d) => {
                    let millis = d.as_millis();
                    self.timeout = Some(Timeout::Instant(time::Instant::now() + d));
                    millis as u32
                }
                Timeout::Instant(instant) => {
                    let dur = instant.saturating_duration_since(time::Instant::now());
                    dur.as_millis() as u32
                }
            };

            let mut params = z3::Params::new(self.context);
            params.set_u32("timeout", millis);

            solver.set_params(&params);
        }

        solver
    }

    fn is_invalid_connection(&self, i: u32, j: u32) -> bool {
        debug_assert!(
            i < self.locations.inputs.len() as u32
                + self.locations.params.len() as u32
                + self.locations.results.len() as u32
                + 1 // Output.
        );
        debug_assert!(
            j < self.locations.inputs.len() as u32
                + self.locations.params.len() as u32
                + self.locations.results.len() as u32
                + 1 // Output.
        );
        self.invalid_connections.contains(&(i, j)) || self.invalid_connections.contains(&(j, i))
    }
    

    //要为每个部件都生成这样一个bitvec的数组，但是在运行的过程中数组的dims会有变化，因此该怎样
    // 目前想法，产生一个很大的bitvec二维数组，此处默认是30*30，通过dims来控制，然后新添加一个变量dims[2]，用来表示这个数组中的哪些元素是有用的
    fn fresh_immediates(&self, bit_width: u32, dims : [usize; 2]) -> Vec<Vecs<Int<'a>>> {
        self.library
            .components
            .iter()
            .flat_map(|c| {
                (0..c.immediate_arity()).map(|_| fresh_immediate(self.context, bit_width, dims))
            })
            .collect()
    }

    fn fresh_param_vars(&self, bit_width: u32, dims : [usize; 2]) -> Vec<Vecs<Int<'a>>> {
        self.library
            .components
            .iter()
            .flat_map(|c| (0..c.operand_arity()).map(|_| fresh_param(self.context, bit_width, dims)))
            .collect()
    }


    fn fresh_result_vars(&self, bit_width: u32, dims : [usize; 2]) -> Vec<Vecs<Int<'a>>> {
        self.library
            .components
            .iter()
            .map(|_| fresh_result(self.context, bit_width, dims))
            .collect()
    }

    fn _add_invalid_assignment(&mut self, assignments: &Assignments) {
        // TODO: like souper, we should have multiple cases here for if we're
        // trying to synthesize any constants or not. When we're synthesizing
        // constants, allow reusing the same location assignments N times with
        // different constants before completely abandoning these location
        // assignments.

        let params = and(
            self.context,
            &assignments
                .params
                .iter()
                .zip(&self.locations.params)
                .map(|(assignment, location)| {
                    location._eq(&self.locations.line_from_u32(self.context, *assignment as _))
                })
                .collect::<Vec<_>>(),
        );

        let results = and(
            self.context,
            &assignments
                .results
                .iter()
                .zip(&self.locations.results)
                .map(|(assignment, location)| {
                    location._eq(&self.locations.line_from_u32(self.context, *assignment as _))
                })
                .collect::<Vec<_>>(),
        );

        let not_this_assignment = and(self.context, &[results, params]).not();
        self.not_invalid_assignments = z3::ast::Bool::<'_>::and(self.context, &[&self.not_invalid_assignments, &not_this_assignment]);
        //self.not_invalid_assignments = self.not_invalid_assignments.and(&[&not_this_assignment]);
    }

    fn reset_invalid_assignments(&mut self) {
        self.not_invalid_assignments = Bool::from_bool(self.context, true);
    }


    //111111
    fn finite_synthesis(
        &mut self,
        input: &Vec<&Vec<Vec<i64>>>,
        output_line: u32,
        bit_width: u32,
    ) -> Result<Assignments> {
        /*debug!(
            "finite synthesis at bit width {} with inputs = {:#018X?}",
            bit_width,
            {
                let mut inputs: Vec<_> = inputs.iter().collect();
                inputs.sort();
                inputs
            }
        );*/

        // 控制dims大于所有可能的size


        let dims = [4, 10];

        let immediates = self.fresh_immediates(bit_width, dims);
        
        //let mut works_for_inputs = Vec::with_capacity(inputs.len() * 4);
        //现在就一组输入直接就是4，用来存储bool们
        let mut works_for_inputs = Vec::with_capacity(3);

        
        let params = self.fresh_param_vars(bit_width, dims);
        let results = self.fresh_result_vars(bit_width, dims);

        
        //将Vec<Vec<Vec<i64>>>类型的inputs转化为Vec<Vecs<Int<'_>>>,
        let input_iter = input.iter();
        let mut inputs : Vec<Vecs<Int<'_>>> = Vec::new();
        for v in input_iter {
            //v type Vec<Vec<i64>>
            let sx = v.len();
            let sy = v[0].len();

            let mut temp : Vecs<Int<'_>> = Vecs::new([Int::from_i64(&self.context, sx as i64), Int::from_i64(&self.context, sy as i64)]);
            for i in 0..sx {
                for j in 0..sy {
                    temp.vecs[i].push(Int::from_i64(self.context, v[i][j] as i64));
                }
            }
            // println!("{:?}", temp);
            inputs.push(temp);
        }
            
        /*let inputs: Vec<_> = input
            .iter()
            .map(|i| 
                (0..i.vecs.len())
                .map(|_| BitVec::from_i64(self.context, i[0] as i64, bit_width))
                .collect()
            ).collect();*/
            
        //i[0] ?
        let output = fresh_output(self.context, bit_width, dims);
        ////用library中components按顺序构造出语句
        let lib = self.library(&immediates, &params, &results, bit_width);
        //println!("lib : {}", lib);
        works_for_inputs.push(lib);

        //建立行数和值之间的关系
        let conn = self.connectivity(&inputs, &output, &params, &results);
        //println!("conn : {}", conn);
        works_for_inputs.push(conn);
        

        let spec = self
            .spec
            .make_expression(self.context, &inputs, &output, bit_width);
        //println!("spec : {}", spec);
        works_for_inputs.push(spec);
        

        let works_for_inputs: Vec<&_> = works_for_inputs.iter().collect();

        assert!(self.spec.arity() <= output_line as usize);
        assert!((output_line as usize) < self.spec.arity() + self.library.components.len());
        let output_on_line = self
            .locations
            .output
            ._eq(&self.locations.line_from_u32(self.context, output_line));

        // let query = self
        //     .well_formed_program
        //     .and(&works_for_inputs)
        //     .and(&[&self.not_invalid_assignments, &output_on_line]);
        
        let query = z3::ast::Bool::<'_>::and(self.context, &[&self.well_formed_program, 
            &(z3::ast::Bool::<'_>::and(self.context, &works_for_inputs)),
            &(z3::ast::Bool::<'_>::and(self.context, &[&self.not_invalid_assignments, &output_on_line])),
            ]);

        trace!("finite synthesis query =\n{:?}", query);

        let solver = self.solver();
        solver.assert(&query);

        match solver.check() {
            z3::SatResult::Unknown => Err(Error::SynthesisUnknown),
            z3::SatResult::Unsat => Err(Error::SynthesisUnsatisfiable),
            z3::SatResult::Sat => {
                let model = solver.get_model();


                // let x : i64 = -10;
                // let x_bv = Int::from_i64(self.context, x);
                // println!(" bitvec :{}", bit_width);
                // println!("bv : {}", x_bv);
                // let y =  eval_bitvec(&Option::as_ref(&model).unwrap(), &x_bv);
                // println!(" y :{}", y);
                

                let immediates = eval_bitvecs(&Option::as_ref(&model).unwrap(), &immediates);

                let params = eval_lines(&Option::as_ref(&model).unwrap(), &self.locations.params);

                let results = eval_lines(&Option::as_ref(&model).unwrap(), &self.locations.results);

                let assignments = Assignments {
                    immediates,
                    params,
                    results,
                    output: output_line,
                };

                debug!(
                    "finite synthesis generated:\n{}",
                    assignments.to_program(self.spec.arity(), &self.library)
                );

                Ok(assignments)
            }
        }
    }

    /*fn verification(&mut self, assignments: &Assignments, bit_width: u32, array_dims: Vec<usize>) -> Result<Verification> {
        let inputs: Vec<_> = (0..self.spec.arity())
            .map(|_| fresh_input(self.context, bit_width, array_dims))
            .collect();
        let output = fresh_output(self.context, bit_width, array_dims);

        let mut prog = assignments.to_program(self.spec.arity(), self.library);
        prog.dce();
        let prog = prog.make_expression(self.context, &inputs, &output, bit_width);

        let spec = self
            .spec
            .make_expression(self.context, &inputs, &output, bit_width);
        let not_spec = spec.not();
        let query = prog.and(&[&not_spec]);
        trace!("verification query =\n{:?}", query);

        let solver = self.solver();
        solver.assert(&query);

        match solver.check() {
            z3::SatResult::Unknown => Err(Error::SynthesisUnknown),
            // There are no more inputs that don't satisfy the spec! We're done!
            z3::SatResult::Unsat => {
                debug!(
                    "verified to work for all inputs at bit width = {}",
                    bit_width
                );
                Ok(Verification::WorksForAllInputs)
            }
            // There still exist inputs for which the synthesized program does
            // not fulfill the spec.
            z3::SatResult::Sat => {
                let model = solver.get_model();
                self.add_invalid_assignment(assignments);
                let inputs = eval_bitvecs(&model, &inputs);
                debug!("found a counter-example: {:?}", inputs);
                Ok(Verification::Counterexample(inputs))
            }
        }
    }*/

    /// 5.2 Encoding Dataflow in Programs
    fn connectivity(
        &self,
        inputs: &Vec<Vecs<Int<'a>>>,
        output: &Vecs<Int<'a>>,
        params: &Vec<Vecs<Int<'a>>>,
        results: &Vec<Vecs<Int<'a>>>,
    ) -> Bool<'a> {
        let locs_to_vars: Vec<_> = self
            .locations
            .inputs
            .iter()
            .zip(inputs)
            .chain(self.locations.params.iter().zip(params))
            .chain(self.locations.results.iter().zip(results))
            .chain(Some((&self.locations.output, output)))
            .collect();

        //println!("location_len : {:?}", locs_to_vars);

        let mut conn =
            Vec::with_capacity(locs_to_vars.len() * locs_to_vars.len() + locs_to_vars.len());

        for (i, (l_x, x)) in locs_to_vars.iter().enumerate() {
            for (j, (l_y, y)) in locs_to_vars.iter().enumerate().skip(i + 1) {
                if self.is_invalid_connection(i as u32, j as u32) {
                    continue;
                }

                //这边默认x和y的len相等
                //判断类型为Vecs<BV<'_>>的x和y中的元素相等关系
                let mut temp = x.vecs[0][0]._eq(&y.vecs[0][0]);
                for i in 0..DIMS[0] {
                    for j in 0..DIMS[1] {
                        let temp2 = x.vecs[i][j]._eq(&y.vecs[i][j]);
                        temp = z3::ast::Bool::<'_>::and(self.context, &[&temp, &temp2]);
                        //temp = temp.and(&[&temp2]);

                    }
                }
                conn.push(l_x._eq(l_y).implies(&temp));

                // let mut temp = x[0]._eq(&y[0]);
                // for _k in 1..x.len() + 1 {
                //     temp = temp.and(&[&temp]);
                // }

                // conn.push(l_x._eq(l_y).implies(&temp));
            }
        }
        //println!("conn : {:?}", conn);

        and(self.context, &conn)
    }

    /* here
    fn library(
        &self,
        immediates: &[BitVec<'a>],
        params: &[BitVec<'a>],
        results: &[BitVec<'a>],
        bit_width: u32,
    ) -> Bool<'a> {
        let mut exprs = Vec::with_capacity(self.library.components.len());
        let mut immediates = immediates;
        let mut params = params;
        let mut results = results.iter();

        for c in &self.library.components {
            let (imms, rest) = immediates.split_at(c.immediate_arity());
            immediates = rest;

            let (inputs, rest) = params.split_at(c.operand_arity());
            params = rest;

            let result = results.next().unwrap();

            exprs.push(
                c.make_expression(self.context, imms, inputs, bit_width)
                    ._eq(result),
            );
        }

        and(self.context, &exprs)
    }*/

    fn library(
        &self,
        immediates: &[Vecs<Int<'a>>],
        params: &[Vecs<Int<'a>>],
        results: &[Vecs<Int<'a>>],
        bit_width: u32,
    ) -> Bool<'a> {
        let mut exprs = Vec::with_capacity(self.library.components.len());
        let mut immediates = immediates;
        let mut params = params;
        let mut results = results.iter();

        for c in &self.library.components {
            let (imms, rest) = immediates.split_at(c.immediate_arity());
            immediates = rest;

            let (inputs, rest) = params.split_at(c.operand_arity());
            params = rest;

            let result = results.next().unwrap();

            let expression = c.make_expression(self.context, imms, inputs, bit_width);

            let sz1 = DIMS[0];
            let sz2 = DIMS[1];

            for i in 0..sz1 {
                for j in 0 .. sz2 {
                    exprs.push(expression.vecs[i][j]._eq(&result.vecs[i][j]));
                }
            }

            
            // exprs.push(
            //     c.make_expression(self.context, imms, inputs, bit_width)[0]
            //         ._eq(&result[0]),
            // );
        }

        //println!("exprs : {:?}", exprs);
        and(self.context, &exprs)
    }

    /// Have the solver generate initial concrete inputs for finite synthesis by
    /// negating the specification.
    ///
    /// Originally, I was using an RNG to generate random initial inputs, but I
    /// took this technique from Souper. Presumably it lets the solver choose
    /// inputs that are more interesting than an RNG would have chosen, which
    /// later helps it synthesize better solutions more quickly.
    /*fn initial_concrete_inputs(&mut self, array_dims: Vec<usize>) -> Result<HashSet<Vec<Vecs<u64>>>> {
        // Taken from Souper.
        const NUM_INITIAL_INPUTS: usize = 4;

        //返回值参数用hashset保证其不重复，原先hashset的参数是Vec<Vec<u64>>，代表生成NUM_INITIAL_INPUTS组输入，每组输入是self.spec.arity()个包含了arraylen个元素的一维数组
        //现在修改完后，需要返回的是生成NUM_INITIAL_INPUTS组输入，每组输入是self.spec.arity()个Vecs变量，因此返回值是HashSet<Vec<Vecs<u64>>>

        let mut inputs: HashSet<Vec<Vecs<u64>>> = HashSet::with_capacity(NUM_INITIAL_INPUTS);
        

        let input_vars : Vec<Vecs<_>> = (0..self.spec.arity())
            .map(|_| fresh_input(self.context, FULL_BIT_WIDTH, array_dims))
            .collect();
        let output_var = fresh_output(self.context, FULL_BIT_WIDTH, array_dims);
        let spec =
            self.spec
                .make_expression(self.context, &input_vars, &output_var, FULL_BIT_WIDTH);
        // let not_spec = spec.not();

        //因为vec<>没有实现copy，只能是传递，因此先构造一个引用
        let in1 = &input_vars;
        let in2 = &input_vars;


        //每次一次循环，产生一个<Vec<Vecs<u64>>放到inputs中，
        for _ in 0..NUM_INITIAL_INPUTS {
            // Make sure that we don't find the same concrete inputs that we've
            // already found.
            let mut existing_inputs = Vec::with_capacity(inputs.len());
            for input_set in &inputs {
                //对于每一组输出，可能有好几个数组作为输入
                let mut this_input = Vec::with_capacity(self.spec.arity());
                for (inp, var) in input_set.iter().zip(in1) {
                    //遍历inp中的实际Vecs.vesc数组，判断是否相等
                    //TODO : 二维到多维
                    let mut temp = BitVec::from_i64(self.context, inp.vecs[0][0] as i64, FULL_BIT_WIDTH)._eq(&var.vecs[0][0]);
                    for i in 0 .. inp.dims[0] {
                        for j in 0 .. inp.dims[1] {
                            let temp2 = BitVec::from_i64(self.context, inp.vecs[i][j] as i64, FULL_BIT_WIDTH)._eq(&var.vecs[i][j]);
                            temp = temp.and(&[&temp2]);
                        }
                    }
                    this_input.push(temp);

                    /*let inp = BitVec::from_i64(self.context, *inp as i64, FULL_BIT_WIDTH);
                    this_input.push(inp._eq(var));*/
                    /*let sz = inp.len();
                    let mut temp = BitVec::from_i64(self.context, inp[0] as i64, FULL_BIT_WIDTH)._eq(&var[0]);
                    for i in 2..sz+1 {
                        let temp2 = BitVec::from_i64(self.context, inp[i-1] as i64, FULL_BIT_WIDTH)._eq(&var[i-1]);
                        temp = temp.and(&[&temp2]);
                    }
                    this_input.push(temp);*/

                }
                let this_input = and(self.context, &this_input);
                existing_inputs.push(this_input);
            }
            let existing_inputs = or(self.context, &existing_inputs);
            let not_existing_inputs = existing_inputs.not();

            let query = spec.and(&[&not_existing_inputs]);
            trace!("initial concrete input synthesis query =\n{:?}", query);

            let solver = self.solver();
            solver.assert(&query);

            match solver.check() {
                z3::SatResult::Unknown => return Err(Error::SynthesisUnknown),
                z3::SatResult::Unsat => return Err(Error::SynthesisUnsatisfiable),
                z3::SatResult::Sat => {
                    let model = solver.get_model();
                    let new_inputs = eval_bitvecs(&model, in2);
                    let is_new = inputs.insert(new_inputs);
                    assert!(is_new);
                }
            }
        }

        Ok(inputs)
    }*/

    /// Synthesize a program!
    ///
    /// The synthesizer has been configured, and we're ready to create a
    /// program.
    pub fn synthesize(&mut self) -> Result<Program> {
        // 将输入从产生随即的输入改称我们自己的输入
        //let mut inputs = self.initial_concrete_inputs(array_dims)?;
        //assert!(!inputs.is_empty());

        //直接获取输入
        let mut input = self.spec.inputs();

        let arity = self.spec.arity();
        assert!(arity > 0);

        let longest = arity as u32 + self.library.components.len() as u32;
        let shortest = if self.should_synthesize_minimal_programs {
            arity as u32 + 1
        } else {
            longest
        };

        // In practice, the cost of searching for a program of length `n` and
        // failing seems to be much more expensive than when there actually is a
        // solution. Therefore, search for the longest programs first and the
        // shortest last. Because we have dead code elimination, we can also
        // skip ahead a bunch of iterations when we find long solutions that
        // contain dead code.
        let mut best = Err(Error::SynthesisUnknown);
        let mut length = longest;
        while length >= shortest {
            //总共一组输入
            match self.synthesize_with_length(length, &mut input) {
                Ok(mut program) => {
                    program.dce();

                    assert!(program.instructions.len() > arity);
                    length = program.instructions.len() as u32 - 1;

                    best = Ok(program);

                    // Reset the invalid-assignments clause, since an assignment
                    // that was an invalid program of length `i` might be valid
                    // at length `i-1`.
                    self.reset_invalid_assignments();

                    continue;
                }
                err => return best.or_else(|_| err),
            }
        }

        best
    }

    fn synthesize_with_length(
        &mut self,
        program_length: u32,
        input: &Vec<&Vec<Vec<i64>>>
    ) -> Result<Program> {
        debug!("synthesizing a program of length = {}", program_length);

        let bit_width = 64;

        //只有一组输入，所以也没有cegis的循环了
        let assignments = self.finite_synthesis(input, program_length - 1, bit_width)?;

        return Ok(assignments.to_program(self.spec.arity(), self.library));


        /*'cegis: loop {
            let assignments = self.finite_synthesis(input, program_length - 1, bit_width)?;

            let mut verifying_with_more_bits = false;
            loop {
                debug!("verifying at bit width = {}", bit_width);
                match self.verification(&assignments, bit_width, array_dims)? {
                    Verification::WorksForAllInputs => {
                        debug_assert!(bit_width <= FULL_BIT_WIDTH);
                        debug_assert!(bit_width.is_power_of_two());
                        if bit_width == FULL_BIT_WIDTH {
                            return Ok(assignments.to_program(self.spec.arity(), self.library));
                        } else {
                            bit_width *= 2;
                            verifying_with_more_bits = true;
                            // TODO: if the synthesized assignments use
                            // immediate constants, try to extend the constants
                            // for the wider bit width in various ways. See
                            // *Program Synthesis for Program Analysis*.
                        }
                    }
                    Verification::Counterexample(new_inputs) => {
                        let is_new = inputs.insert(new_inputs);
                        assert!(is_new || verifying_with_more_bits);
                        continue 'cegis;
                    }
                }
            }
        }*/
    }
}


impl Program {
    pub fn synthesize<'a>(
        context: &'a z3::Context,
        spec: &impl Specification,
        library: &Library,
        _arr_dims : Vec<usize>
    ) -> Result<Program> {
        let mut synthesizer = Synthesizer::new(context, library, spec)?;
        synthesizer.synthesize()
    }

    pub fn dce(&mut self) {
        let mut used: HashSet<Id> = HashSet::from_iter(
            self.instructions
                .iter()
                .take_while(|inst| inst.operator == Operator::Var)
                .map(|inst| inst.result)
                .chain(Some(self.instructions.last().unwrap().result)),
        );

        for inst in self.instructions.iter().rev() {
            if !used.contains(&inst.result) {
                continue;
            }

            inst.operator.operands(|op| {
                used.insert(op);
            });
        }

        self.instructions.retain(|inst| used.contains(&inst.result));

        let mut renumbering = HashMap::new();
        for (i, inst) in self.instructions.iter_mut().enumerate() {
            inst.operator.operands_mut(|x| {
                *x = renumbering[x];
            });

            let old = renumbering.insert(inst.result, Id(i as u32));
            debug_assert!(old.is_none());
            inst.result = Id(i as u32);
        }
    }
}


impl Specification for Program {
    fn arity(&self) -> usize {
        self.instructions
            .iter()
            .take_while(|inst| inst.operator == Operator::Var)
            .count()
    }

    fn inputs(&self) -> Vec<&Vec<Vec<i64>>> {
        let inputs : Vec<_> = self.inputs
        .iter()
        .clone()
        .collect();
     
        return inputs;
    }

    fn make_expression<'a>(
        &self,
        context: &'a z3::Context,
        inputs: &Vec<Vecs<Int<'a>>>,
        output: &Vecs<Int<'a>>,
        bit_width: u32,
    ) -> Bool<'a> {
        assert!(self.instructions.len() > inputs.len());

        let mut vars: Vec<_> = inputs.iter().cloned().collect();

        //测试输入有没有被正确传达到
        // let x : Vec<_> = vars.iter().clone().collect();
        // println!("inputs : {:?}", x);

        let mut operands = vec![];
        for instr in self.instructions.iter().skip(inputs.len()) {
            // NB: programs cannot contain unbound constants, so specifications
            // constructed from programs will never require us to synthesize an
            // immediate.
            let immediates = [];

            operands.clear();
            instr
                .operator
                .operands(|Id(x)| operands.push(vars[x as usize].clone()));

            vars.push(
                instr
                    .operator
                    .make_expression(context, &immediates, &operands, bit_width),
            );
            
            // let x : Vec<_> = vars.iter().clone().collect();
            // println!("vars : {:?}", x);
        }

        //最后的结果
        let vars = vars.pop().unwrap();
        // let x  = vars.clone();
        // println!("vars : {:?}", x);

        // let x  = output.clone();
        // println!("output : {:?}", output);

        //利用vars和output中的元素相等构成逻辑表达式
        let mut temp: Bool<'_> = vars.vecs[0][0]._eq(&output.vecs[0][0]);

        // println!("temp0 : {}", temp);
        for i in 0..DIMS[0] {
            for j in 0..DIMS[1] {
                let temp2 = vars.vecs[i][j]._eq(&output.vecs[i][j]);
                //temp = temp.and(&[&temp2]);
                temp = z3::ast::Bool::<'_>::and(&context, &[&temp, &temp2]);
            }
        }
        

        // let mut temp = vars[0]._eq(&output[0]);

        // for i in 2..vars.len() {
        //     //println!("{}", vars.len());
        //     let temp2 = vars[i-1]._eq(&output[i-1]);
        //     temp = temp.and(&[&temp2]);
        // }

        // println!("temp : {}", temp);
        
        return temp;    
    }

}
/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_id() {
        assert_eq!(Id(0).to_string(), "a");
        assert_eq!(Id(1).to_string(), "b");
        assert_eq!(Id(2).to_string(), "c");
        assert_eq!(Id(25).to_string(), "z");

        assert_eq!(Id(26).to_string(), "aa");
        assert_eq!(Id(27).to_string(), "ab");
        assert_eq!(Id(28).to_string(), "ac");
        assert_eq!(Id(51).to_string(), "az");

        assert_eq!(Id(52).to_string(), "ba");
        assert_eq!(Id(53).to_string(), "bb");
        assert_eq!(Id(54).to_string(), "bc");

        assert_eq!(Id(0 * 26 + 1 * 26 + 26 * 26 - 1).to_string(), "zz");
        assert_eq!(Id(0 * 26 + 1 * 26 + 26 * 26).to_string(), "aaa");
    }

    #[test]
    fn display_operator() {
        assert_eq!(Operator::Mul(Id(1), Id(2)).to_string(), "mul b, c");
    }

    #[test]
    fn display_instruction() {
        let instr = Instruction {
            result: Id(3),
            operator: Operator::And(Id(1), Id(2)),
        };
        assert_eq!(instr.to_string(), "d ← and b, c");
    }

    #[test]
    fn display_program() {
        let mut builder = ProgramBuilder::new();
        let a = builder.var();
        let b = builder.var();
        let c = builder.const_(1);
        let d = builder.add(a, c);
        let _e = builder.and(b, d);
        let program = builder.finish();
        assert_eq!(
            program.to_string(),
            "a ← var\n\
             b ← var\n\
             c ← const 0x1\n\
             d ← add a, c\n\
             e ← and b, d\n\
             "
        );
    }

    #[test]
    fn synthesize() {
        let mut config = z3::Config::new();
        config.set_model_generation(true);

        let context = z3::Context::new(&config);

        let library = Library::brahma_std();
        let mut builder = ProgramBuilder::new();
        let a = builder.var();
        let b = builder.const_(2);
        let _ = builder.mul(a, b);
        let spec = builder.finish();

        let mut p = Program::synthesize(&context, &spec, &library).unwrap();
        p.dce();
        println!("{}", p.to_string());
    }

    #[test]
    fn synthesize_select() {
        let mut config = z3::Config::new();
        config.set_model_generation(true);

        let context = z3::Context::new(&config);

        let mut library = Library::brahma_std();
        library.components.push(component::select());

        let mut builder = ProgramBuilder::new();
        let a = builder.var();
        let b = builder.var();
        let c = builder.var();
        let _ = builder.select(a, b, c);
        let spec = builder.finish();

        let mut p = Program::synthesize(&context, &spec, &library).unwrap();
        p.dce();
        println!("{}", p.to_string());
    }
}*/