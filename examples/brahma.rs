use std::vec;

use structopt::*;
use synth_loop_free_prog::{Result as SynthResult, *};

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
            "test_add".to_string(),
            "test_cast".to_string(),
            "duplicate_test_add".to_string(),
            "simple_broadcasted_add".to_string(),
            "simple_with_input_names".to_string(),
            "simple_cast".to_string(),
            "simple_sparse_add".to_string(),
            "simple_add_big_tensors".to_string(),
            "simple_using_constant".to_string(),
            "simple_using_output_shape".to_string(),
            "simple_using_output_shape_tuple".to_string(),
            "simple_using_primitive_input".to_string(),
            "stackoverflow_02".to_string(),
            "stackoverflow_06".to_string(),
            "stackoverflow_11".to_string(),
            "stackoverflow_15".to_string(),
            "stackoverflow_16".to_string(),
            "stackoverflow_35".to_string(),
            "stackoverflow_39".to_string(),
            "stackoverflow_48".to_string(),
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
        test_cast,
        duplicate_test_add,
        simple_broadcasted_add,
        simple_with_input_names,
        simple_cast,
        simple_sparse_add,
        simple_add_big_tensors,
        simple_using_constant,
        simple_using_output_shape,
        simple_using_output_shape_tuple,
        simple_using_primitive_input,
        stackoverflow_02,
        stackoverflow_06,
        stackoverflow_11,
        stackoverflow_15,
        stackoverflow_16,
        stackoverflow_35,
        stackoverflow_39,
        stackoverflow_48,
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
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![20]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// test_cast
fn test_cast(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![1, 0, 1, 1, 0]);

    let in1 = builder.var(input1);

    let _ = builder.tf_cast(in1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// test_inconsistent_target_program 不用管这个，这个是错误的样例

// duplicate_test_add
fn duplicate_test_add(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![20]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}
  
// simple——benchmarks

// simple_broadcasted_add
fn simple_broadcasted_add(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![3, 4, 5]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![10, 20, 30]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_expand_dims(in2);
    let _ = builder.tf_add(in1, o1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// simple_with_input_names
fn simple_with_input_names(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![3, 4, 5]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![10, 20, 30]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_expand_dims(in2);
    let _ = builder.tf_add(in1, o1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// simple_cast
fn simple_cast(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![12, 34, 56]);

    let in1 = builder.var(input1);

    let _ = builder.tf_cast(in1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

//todo simple_index 直接数组下标操作 in1[in2]

//todo simple_slice 也是数组下标切片问题

// simple_sparse_add
// 暂时先不管啥稠密张量稀疏张量，能满足二维数组就行，并且有些还不符合要求呢
fn simple_sparse_add(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![12, 34]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![-3, 0]);    
    input2.push(vec![-5, 0]); 

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// simple_add_big_tensors
// 我们的数组是4*10的，所以把超过4（第二个输入要expanddims所以看最多4个）的部分砍掉
fn simple_add_big_tensors(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1000, 2000, 3000, 4000]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_expand_dims(in2);
    let _ = builder.tf_add(in1, o1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// simple_using_constant
// 目前先用变量代替常量
fn simple_using_constant(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 2, 3]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![100]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_constant(in2);
    let _ = builder.tf_add(in1, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// simple_using_output_shape
// 我们的数组是4*10的，所以把5改成了4
fn simple_using_output_shape(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![7]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![4]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let a = builder.tf_eye(in2, in2);
    let _ = builder.tf_multiply(in1, a);

    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// simple_using_output_shape_tuple 
// 我们的数组是4*10的，所以把2, 3, 4, 5改成了2, 3
fn simple_using_output_shape_tuple(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![2, 3]);

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
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![123]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![45]);    

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

//todo simple_with_many_inputs tf.gather

//todo simple_output_equals_input_single 输入=输出，这个测试用例意义不明

//todo simple_output_equals_input_multiple 从几组中挑一个，依旧意义不明

//todo simple_output_equals_constant 意义不明

// google_benchmarks

//todo google_01 tf.sequence_mask tf.concat tf.where

//todo google_02 tf.reduce_sum

//todo google_03 tf.sparse.slice

//todo google_04 tf.reshape

//todo google_05 tf.tile

//todo google_06 tf.math.segment_max

//todo google_07 tf.sequence_mask tf.cumsum

//todo google_08 tf.range

//todo google_09 tf.gather tf.argsort

//google_10
// fn google_10(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library::brahma_std();
//     let mut builder = ProgramBuilder::new();

//     let mut input1 : Vec<Vec<i64>> = Vec::new();
//     input1.push(vec![10, 20, 0, 40, 0, 30]);

 
//     let mut input2 : Vec<Vec<i64>> = Vec::new();
//     input2.push(vec![1, 1, 0, 1, 0, 1]);

//     let in1 = builder.var(input1);
//     let in2 = builder.var(input2);
//     let _ = builder.tf_boolean_mask(in1, in2);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

//todo google_11 tf.reduce_sum 用到了浮点数转换为整数，目前可以将输入手动转为整数

//todo google_12 tf.logical_and 用到了浮点数转换为整数，目前可以将输入手动转为整数

//todo google_13 tf.concat

//todo google_14 tf.roll

//todo google_15 tf.pad

//todo google_16 tf.gather tf.sequence_mask tf.where

//todo google_17 tf.where

//todo google_18 tf.linalg.matvec

//todo google_19 tf.gather th.argsort

//todo_google_20 tf.argsort

//todo google_21 tf.tensor_scatter_nd_update

//todo google_22 tf.where tf.reduce_max tf.one_hot

// stackoverflow_benchmarks

//todo stackoverflow_01 tf.transpose

// stackoverflow_02
fn stackoverflow_02(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![5, 1, 0, 3, 0, -1, 2, -10, 2]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_constant(in2);
    let _ = builder.tf_minimum(in1, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_03 tf.reduce_sum tf.one_hot

//todo stackoverflow_04 tf.gather_nd tf.stack

//todo stackoverflow_05 tf.tensordot

// stackoverflow_06
// 我们的数组是4*10的，所以把3, 5, 0, 2, 3, 3, 0改成3, 5, 0, 2
fn stackoverflow_06(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![3, 5, 0, 2]);

    let in1 = builder.var(input1);

    let o1 = builder.tf_expand_dims(in1);
    let o2 = builder.tf_equal(in1, o1);
    let _ = builder.tf_cast(o2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_07 tf.unstack

//todo stackoverflow_08 tf.boolean_mask

//todo stackoverflow_09 tf.unique_with_counts

//todo stackoverflow_10 tf.matmul

// stackoverflow_11
// 我们的数组是4*10的，所以把4, 0, 1, 1, 0, 4, 0, 0, 3, 4, 1改成4, 0, 1, 1, 0, 4, 0, 0, 3, 4
fn stackoverflow_11(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![4, 0, 1, 1, 0, 4, 0, 0, 3, 4]);

    let in1 = builder.var(input1);

    let _ = builder.tf_bincount(in1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_12 tf.gather

//todo stackoverflow_13 tf.tensordot

//todo stackoverflow_14 tf.reduce_any

// stackoverflow_15
fn stackoverflow_15(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![3, 1, 2, 0, 1, -1, 10, 1, -10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_constant(in2);
    let o2 = builder.tf_equal(in1, o1);
    let o3 = builder.tf_cast(o2);
    let _ = builder.tf_subtract(in1, o3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_16
fn stackoverflow_16(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![2, 5]);
    input1.push(vec![3, 0]);
    input1.push(vec![8, 7]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![4, 10, -6]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_expand_dims(in2);
    let _ = builder.tf_multiply(in1, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_17 tf.stack

//todo stackoverflow_18 tf.matmul

//todo stackoverflow_19 tf.gather tf.argsort

//todo stackoverflow_20 tf.one_hot

//todo stackoverflow_21 tf.gather

//todo stackoverflow_22 tf.tensordot

//todo stackoverflow_23 tf.reduce_max tf.one_hot

//todo stackoverflow_24 tf.where

//todo stackoverflow_25 tf.tile

//todo stackoverflow_26 tf.reduce_sum

//todo stackoverflow_27 tf.reduce_max tf.one_hot

//todo stackoverflow_28 tf.squeeze tf.gather

//todo stackoverflow_29 tf.searchsorted

//todo stackoverflow_30 tf.sqrt tf.reduce_sum

//todo stackoverflow_31 tf.reduce_sum tf.sparse.to_dense

//todo stackoverflow_32 tf.tensordot tf.range

//todo stackoverflow_33 tf.reduce_min tf.reduce_sum

//todo stackoverflow_34 tf.tensordot

// stackoverflow_35
// 我们的数组是4*10的，所以把[[[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]], [[10., 20.], [30., 40.], [50., 60.]]], 
// [[[9.0, 8.0], [7.0, 6.0], [5.0, 4.0]], [[90., 80.], [70., 60.], [50., 40.]]] 改成
// [1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [10., 20.]和[9.0, 8.0], [7.0, 6.0], [5.0, 4.0], [90., 80.]
fn stackoverflow_35(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 2]);
    input1.push(vec![3, 4]);
    input1.push(vec![5, 6]);
    input1.push(vec![10, 20]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![9, 8]);
    input2.push(vec![7, 6]);
    input2.push(vec![5, 4]);
    input2.push(vec![90, 80]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![1, 4, 8]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_expand_dims(in3);
    let o2 = builder.tf_subtract(in1, in2);
    let o3 = builder.tf_multiply(o1, o2);
    let _ = builder.tf_add(in2, o3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_36 tf.range

//todo stackoverflow_37 tf.tensordot

//todo stackoverflow_38 tf.reduce_prod tf.reduce_max

// stackoverflow_39
// 由于类型只有整数，并且不允许多次调用同一个表达式，因此自行优化[[-1.5, 1.0, 0.9, 2.0], [1.1, 0.0, -0.1, -0.9], [-1.0, 0.1, -1.1, 2.5]]为
// [[-15, 1, 0, 2], [1, 0, 0, 0], [-1, 0, -11, 25]]
fn stackoverflow_39(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![-15, 1, 0, 2]);
    input1.push(vec![1, 0, 0, 0]);
    input1.push(vec![-1, 0, -11, 25]);

    let in1 = builder.var(input1);

    let o1 = builder.tf_cast(in1);
    let o2 = builder.tf_square(o1);
    let _ = builder.tf_multiply(o2, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_40 tf.sparse.to_dense

//todo stackoverflow_41 tf.boolean_mask tf.range

//todo stackoverflow_42 tf.reduce_max

//todo stackoverflow_43 tf.gather_nd tf.transpose

//todo stackoverflow_44 tf.squeeze tf.reduce_sum tf.reshape

//todo stackoverflow_45 tf.where tf.sequence_mask tf.roll

//todo stackoverflow_46 tf.where tf.sequence_mask

//todo stackoverflow_47 tf.reshape tf.gather tf.cumsum tf.reshape

// stackoverflow_48
fn stackoverflow_48(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![32, 53, 45, 38, 29, 89, 64, 23]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![38, 53, 89, 38, 32, 64]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_expand_dims(in2);
    let o2 = builder.tf_equal(in1, o1);
    let o3 = builder.tf_cast(o2);
    let _ = builder.tf_argmax(o3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_49 tf.transpose

//todo stackoverflow_50 tf.one_hot