use std::vec;

use structopt::*;
use synth_loop_free_prog::{Result as SynthResult, *, component::tf_eye};

macro_rules! benchmarks {
    ( $($name:ident,)* ) => {
        vec![
            $(
                (stringify!($name), $name as _),
            )*
        ]
    }
}

fn main() {
    env_logger::init();

    let mut opts = Options::from_args();
    if opts.mytest {
        opts.problems = vec![
            "test_benchmarks_test_add".to_string(),
            "mytest2".to_string(),
            "mytest3".to_string(),
            "mytest4".to_string(),
            "mytest5".to_string(),
        ];
    }

    let mut config = z3::Config::new();
    config.set_bool_param_value("auto_config", false);
    config.set_model_generation(true);

    let context = z3::Context::new(&config);

    let problems: Vec<(
        &'static str,
        fn(&z3::Context, &Options) -> SynthResult<Program>,
    )> = benchmarks! { 
        test_add,
        // mytest2,
        google_10,
        // mytest4,
        // mytest5,
    };

    for (name, p) in problems {
        if !opts.should_run_problem(name) {
            continue;
        }

        println!("==================== {} ====================", name);
        let then = std::time::Instant::now();
        let program = p(&context, &opts);
        let elapsed = then.elapsed();

        println!(
            "\nElapsed: {}.{:03}s\n",
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
        match program {
            Ok(prog) => {
                println!("Synthesized:\n\n{}", prog);
            }
            Err(e) => {
                println!("Error: {:?}\n", e);
            }
        }
    }
}

#[derive(StructOpt)]
struct Options {
    /// Set a timeout, in milliseconds.
    #[structopt(short = "t", long = "timeout")]
    timeout: Option<u32>,

    /// Synthesize the optimally smallest programs.
    #[structopt(short = "m", long = "minimal")]
    minimal: bool,

    /// Run only the problems that we can solver pretty fast.
    // #[structopt(short = "f", long = "only-fast", conflicts_with = "problems")]
    // only_fast: bool,

    /// Should constants be given or synthesized? It isn't always clear which
    /// they did in the paper, and sort seems like they did a mix depending on
    /// the benchmark problem.
    // #[structopt(short = "c", long = "synthesize-constants")]
    // synthesize_constants: bool,

    /// When supplied, run only these problems instead of all problems.
    #[structopt(last = true)]
    problems: Vec<String>,

    /// 作为自己的一个测试样例，目前处于探索阶段，不破坏程序结构.
    #[structopt(short = "e", long = "mytest", conflicts_with = "problems")]
    mytest: bool,
}

impl Options {
    fn should_run_problem(&self, problem: &str) -> bool {
        self.problems.is_empty() || self.problems.iter().position(|p| p == problem).is_some()
    }
}

fn synthesize(
    opts: &Options,
    context: &z3::Context,
    spec: &dyn Specification,
    library: &Library
) -> SynthResult<Program> {
    Synthesizer::new(context, library, spec)?
        .set_timeout(opts.timeout)
        .should_synthesize_minimal_programs(opts.minimal)
        .synthesize()
}
// test_benchmarks

// test_add
fn test_add(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![20]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    // let z = builder.tf_add(in1, in2);
    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// todo: test_cast

// test_inconsistent_target_program 
// fn test_benchmarks_test_inconsistent_target_program(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

//     let library = Library::brahma_std();
//     let mut builder = ProgramBuilder::new();
    
//     // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
//     // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
     
//     let mut input1 : Vec<Vec<i64>> = Vec::new();   
//     input1.push(vec![10]);

//     let mut input2 : Vec<Vec<i64>> = Vec::new();
//     input2.push(vec![20]);    

//     let in1 = builder.var(input1);
//     let in2 = builder.var(input2);

//     // let z = builder.tf_add(in1, in2);
//     let _ = builder.tf_add(in1, in2);
//     let spec = builder.finish();

//     return synthesize(opts, context, &spec, &library); 
// }

//duplicate_test_add
fn duplicate_test_add(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![20]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    // let z = builder.tf_add(in1, in2);
    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}
  
//------simple——benchmarks------------

//todo simple_broadcasted_add tf.expand_dims

//todo simple_with_input_names tf.expand_dims

//todo simple_cast tf.cast

//todo simple_index 直接数组下标操作 in1[in2]

//todo simple_slice 也是数组下标切片问题

//todo simple_sparse_add tf.sparse.from_dense， tf.spare.add

//todo simple_add_big_tensors tf.expand_dims

//simple_using_constant
// fn simple_benchmarks_simple_using_constant(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     let mut const1 : Vec<Vec<i64>> = Vec::new();
//     const1.push(vec![100]);
//     library
//         .components
//         .push(component::const_(const1.clone()));
//     let mut builder = ProgramBuilder::new();
//     let mut input1 : Vec<Vec<i64>> = Vec::new();
//     input1.push(vec![1,2,3]);

//     let in1 = builder.var(input1);
//     let const100 = builder.const_(const1);
//     let _ = builder.tf_add(in1, const100);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// simple_using_output_shape
fn simple_using_output_shape(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![7]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![5]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let a = builder.tf_eye(in2, in2);
    let _ = builder.tf_mul(in1, a);

    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

//todo simple_using_output_shape_tuple 
fn simple_using_output_shape_tuple(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![2,3,4,5]);

    let in1 = builder.var(input1);

    let _ = builder.tf_zeros(in1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

//todo simple_using_boolean_constant tf.SparseTensor

//todo simple_using_constant_kwarg tf.argsort

// simple_using_primitive_input
fn simple_using_primitive_input(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![123]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![45]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    // let z = builder.tf_add(in1, in2);
    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

//todo simple_with_many_inputs tf.gather

//todo simple_output_equals_input_single 输入=输出，这个测试用例意义不明

//todo simple_output_equals_input_multiple 从几组中挑一个，依旧意义不明

//todo simple_output_equals_constant 意义不明

//-----------google_benchmarks---------

//todo google_01 tf.cast tf.where tf.sequence_mask tf.math.bincount

//todo google_02 tf.expand_dims tf.reduce_sum

//todo google_03 tf.sparse.slice

//todo google_04 tf.reshape

//todo google_05 tf.tile tf.expand_dims

//todo google_06 tf.math.segment_max

//todo google_07 tf.sequence_mask

//todo google_08 tf.expand_dims tf.range

//todo google_09 tf.gather tf.argsort

//google_10
fn google_10(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![10, 20, 0, 40, 0, 30]);

 
    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1, 1, 0, 1, 0, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let _ = builder.tf_boolean_mask(in1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo google_11 tf.reduce_sum tf.cast

//todo google_12 tf.logical_and tf.cast 用到了浮点数

//todo google_13 tf.concat的轴
// fn google_13(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library::brahma_std();
//     let mut builder = ProgramBuilder::new();

//     let mut input1 : Vec<Vec<i64>> = Vec::new();
//     input1.push(vec![1, 2]);
//     input1.push(vec![10, 20]);

//     let mut input2 : Vec<Vec<i64>> = Vec::new();
//     input2.push(vec![3, 4, 5]);
//     input2.push(vec![30, 40, 50]);


 
//     let mut input2 : Vec<Vec<i64>> = Vec::new();
//     input2.push(vec![1, 1, 0, 1, 0, 1]);

//     let in1 = builder.var(input1);
//     let in2 = builder.var(input2);
//     let _ = builder.tf_boolean_mask(in1, in2);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }




