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

    let opts = Options::from_args();

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
        test_inconsistent_target_program,
        duplicate_test_add,
        simple_broadcasted_add,
        simple_with_input_names,
        simple_cast,
        simple_sparse_add,
        simple_add_big_tensors,
        simple_using_constant,
        simple_using_output_shape,
        simple_using_output_shape_tuple,
        simple_using_boolean_constant,
        simple_using_primitive_input,
        simple_output_equals_input_single,
        simple_output_equals_input_multiple,
        simple_output_equals_constant,
        google_01,
        google_02,
        google_06,
        google_08,
        google_10,
        google_11,
        google_12,
        google_13,
        google_14,
        google_17,
        google_18,
        stackoverflow_01,
        stackoverflow_02,
        stackoverflow_05,
        stackoverflow_06,
        stackoverflow_08,
        stackoverflow_11,
        stackoverflow_13,
        stackoverflow_14,
        stackoverflow_15,
        stackoverflow_16,
        stackoverflow_17,
        stackoverflow_20,
        stackoverflow_22,
        stackoverflow_24,
        stackoverflow_26,
        stackoverflow_27,
        stackoverflow_31,
        stackoverflow_32,
        stackoverflow_34,
        stackoverflow_35,
        stackoverflow_36,
        stackoverflow_37,
        stackoverflow_38,
        stackoverflow_39,
        stackoverflow_41,
        stackoverflow_42,
        stackoverflow_45,
        stackoverflow_46,
        stackoverflow_48,
        stackoverflow_50,
        autopandas4,
        autopandas8,
        autopandas10,
        autopandas11,
        autopandas13,
        autopandas14,
        autopandas15,
        autopandas16,
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

    // 作为自己的一个测试样例，目前处于探索阶段，不破坏程序结构.
    // #[structopt(short = "e", long = "mytest", conflicts_with = "problems")]
    // mytest: bool,
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

// test_inconsistent_target_program
// 我们把它手动改成对的不就行了？
fn test_inconsistent_target_program(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

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

//simple_index 直接数组下标操作 in1[in2] 与项目无关，无法实现

//simple_slice 也是数组下标切片问题，与项目无关，无法实现

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

    let mut input3 : Vec<Vec<i64>> = Vec::new();   
    input3.push(vec![4]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let a = builder.tf_eye(in2, in3);
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

// simple_using_boolean_constant
// 将稀疏张量手动设置为连续的张量
fn simple_using_boolean_constant(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![12, 34]);
    input1.push(vec![0, 56]);

    let in1 = builder.var(input1);

    let _ = builder.tf_reduce_sum1(in1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

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

// simple_with_many_inputs 维度超过二维，无法实现

// simple_output_equals_input_single
// 直接相等，我们采用constant来等价
fn simple_output_equals_input_single(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![10, 20, 30, 40, 50]);

    let in1 = builder.var(input1);

    let _ = builder.tf_constant(in1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// simple_output_equals_input_multiple
// 我们自己改成多个输入不就好了？
fn simple_output_equals_input_multiple(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![1, 2, 3, 4, 5]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![10, 20, 30, 40, 50]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();   
    input3.push(vec![100, 200, 300]);

    let _in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let _in3 = builder.var(input3);

    let _ = builder.tf_constant(in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// simple_output_equals_constant
// 我们自己改成多个输入找常量不就好了？
fn simple_output_equals_constant(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![1, 2, 3, 4, 5]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![10, 20, 30, 40, 50]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();   
    input3.push(vec![100, 200, 300]);

    let mut input4 : Vec<Vec<i64>> = Vec::new();   
    input4.push(vec![10]);

    let _in1 = builder.var(input1);
    let _in2 = builder.var(input2);
    let _in3 = builder.var(input3);
    let in4 = builder.var(input4);

    let _ = builder.tf_constant(in4);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// google_benchmarks

// google_01
// 原来的样例超出4*10范围，将[0, 0, 0, 1, 3, 3]改成[0, 0, 0, 1]
fn google_01(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![0, 0, 0, 1]);

    let in1 = builder.var(input1);

    let o1 = builder.tf_bincount(in1);
    let o2 = builder.tf_sequence_mask(o1);
    let o3 = builder.tf_where1(o2);
    let _ = builder.tf_cast(o3);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// google_02
// 浮点数手动调整为整数
fn google_02(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![0, 10, 0, 0]);
    input1.push(vec![0, 10, 10, 0]);
    input1.push(vec![10, 10, 10, 10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![0, 10, 0, 0]);
    input2.push(vec![0, 10, 10, 0]);
    input2.push(vec![10, 10, 10, 10]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_reduce_sum1(in1);
    let o2 = builder.tf_expand_dims(o1);
    let _ = builder.tf_divide(in2, o2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// google_03 无法实现，维度已经超过二维

// google_04 无法实现，维度已经超过二维

// google_05 无法实现，维度已经超过二维

// google_06
fn google_06(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![0, 1, 1, 2, 3, 3, 3, 3]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![1, 3, 4, 5, 10, 8, 9, 4]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_segment_max(in2, in1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

//todo google_07 出现多次，暂时无法实现

// google_08
fn google_08(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![3, 4, 2, 1]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![0]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();   
    input3.push(vec![5]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_expand_dims(in1);
    let o2 = builder.tf_range(in2, in3);
    let o3 = builder.tf_greater(o1, o2);
    let _ = builder.tf_cast(o3);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

//todo google_09 tf.gather tf.argsort

// google_10
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

// google_11 
// 用到了浮点数转换为整数，目前可以将输入手动转为整数
fn google_11(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![10, 3, -42, 0, 21, 4]);
    input1.push(vec![-1, 0, 14, -10, 4, 0]);
    input1.push(vec![1, 0, 7, -3, 5, -1]);
    input1.push(vec![14, 25, 3, 1, 0, 12]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![0, 0, 0, 0, 0, 0]);
    input2.push(vec![0, 0, 0, 0, 0, 0]);
    input2.push(vec![0, 0, 0, 0, 0, 0]);
    input2.push(vec![0, 0, 0, 0, 0, 0]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_greater(in1, in2);
    let o2 = builder.tf_cast(o1);
    let _ = builder.tf_reduce_sum1(o2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// google_12
// 用到了浮点数转换为整数，目前可以将输入手动转为整数
// 手动实现logical_and
fn google_12(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![10, 3, -42, 0, 21]);
    input1.push(vec![-1, 0, 14, -10, 4]);
    input1.push(vec![1, 0, 7, -3, 5]);
    input1.push(vec![14, 25, 3, -1, 0]);

    let in1 = builder.var(input1);

    let _ = builder.tf_cast(in1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// google_13
fn google_13(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 2]);
    input1.push(vec![10, 20]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![3, 4, 5]);
    input2.push(vec![30, 40, 50]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_concat1(in1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// google_14
fn google_14(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 3, 2, 0, 0]);
    input1.push(vec![4, 6, 5, 0, 0]);
    input1.push(vec![8, 7, 9, 0, 0]);

    let in1 = builder.var(input1);

    let _ = builder.tf_roll(in1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo google_15 tf.pad

//todo google_16 tf.gather

// google_17
// 输入的布尔值手动改为整数
fn google_17(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 0, 0, 1, 0]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1, 2, 3, 4, 5]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![-10, -10, -10, -10, -10]);

    let mut input4 : Vec<Vec<i64>> = Vec::new();
    input4.push(vec![1, 2, 3, 4, 5]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);
    let in4 = builder.var(input4);

    let o1 = builder.tf_multiply(in2, in3);
    let _ = builder.tf_where3(in1, in4, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// google_18
// 采用等价的方式进行处理
// 超出4*10范围，[5, 7, -12, 10, 20], [1, 2, 3, 1, 2]改为[5, 7, -12, 10], [1, 2, 3, 1]
fn google_18(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![5, 7, -12, 10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1, 2, 3, 1]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![1, 2, 3, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_expand_dims(in2);
    let o2 = builder.tf_equal(o1, in3);
    let o3 = builder.tf_cast(o2);
    let _ = builder.tf_matmul(o3, in1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo google_19 tf.gather th.argsort

//todo_google_20 tf.argsort

//todo google_21 tf.tensor_scatter_nd_update

// google_22 无法实现，维度超过二维

// stackoverflow_benchmarks

// stackoverflow_01
// 原本的测试样例是矩阵翻转之后又复制了一遍，现在就不复制了，原本的小数改为整数
fn stackoverflow_01(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![5, 2]);
    input1.push(vec![1, 3]);
    input1.push(vec![0, -1]);

    let in1 = builder.var(input1);

    let o1 = builder.tf_cast(in1);
    let _ = builder.tf_transpose(o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

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

// stackoverflow_03 无法实现，重复多次出现同一表达式，且出现多次非tf运算

//todo stackoverflow_04 tf.gather_nd

// stackoverflow_05
// 原来的第二个输入需要按照列进行遍历，手动改为列遍历后的结果
fn stackoverflow_05(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![4, 3, 1]);
    input1.push(vec![6, 5, 2]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![5, 5]);
    input2.push(vec![1, 5]);
    input2.push(vec![6, 0]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_tensordot(in1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_06
// 我们的数组是4*10的，所以把3, 5, 0, 2, 3, 3, 0改成3, 5, 0, 2
fn stackoverflow_06(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![3, 5, 0, 2]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![3, 5, 0, 2]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_expand_dims(in1);
    let o2 = builder.tf_equal(in2, o1);
    let _ = builder.tf_cast(o2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_07 无法实现，维度已经超过二维

// stackoverflow_08
// 原输入超过十个，手动调整为十个以内,
// [-1, 0, -3, 2, 1, 3, 5, -1, -9, 2, 10]为[-1, 0, -3, 2, 1, 3, 5, -1, -9, 2]
// [12, 3, 45, 6, 7, 8, 9, 87, 65, 4, 32]为[12, 3, 45, 6, 7, 8, 9, 87, 65, 4]
fn stackoverflow_08(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![-1, 0, -3, 2, 1, 3, 5, -1, -9, 2]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![12, 3, 45, 6, 7, 8, 9, 87, 65, 4]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_constant(in3);
    let o2 = builder.tf_greater(in1, o1);
    let _ = builder.tf_boolean_mask(in2, o2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_09 tf.unique_with_counts

// stackoverflow_10 无法实现，维度已经超过二维

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

// stackoverflow_13
// 原本的输入维度高于二维，手动改为二维 [[[1, 0], [5, 4]], [[3, 10], [2, -2]]]改为[[1, 0], [5, 4]]
fn stackoverflow_13(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![3, 5]);
    input1.push(vec![10, 2]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1, 0]);
    input2.push(vec![5, 4]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_tensordot(in1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_14
// 超出4*10范围，[[False, False, True], [False, False, False], [True, False, True], [False, True, False], [False, False, False],
// [True, True, True], [True, True, False]]修改为[[False, False, True], [False, False, False], [True, False, True], [False, True, False]]
// 用1 0代替true false
fn stackoverflow_14(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![0, 0, 1]);
    input1.push(vec![0, 0, 0]);
    input1.push(vec![1, 0, 1]);
    input1.push(vec![0, 1, 0]);

    let in1 = builder.var(input1);

    let _ = builder.tf_reduce_any1(in1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_15
fn stackoverflow_15(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![3, 1, 2, 0, 1, -1, 10, 1, -10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![3, 1, 2, 0, 1, -1, 10, 1, -10]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_constant(in2);
    let o2 = builder.tf_equal(in1, o1);
    let o3 = builder.tf_cast(o2);
    let _ = builder.tf_subtract(in3, o3);
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

// stackoverflow_17
// tf.stack和tf.concat等价
fn stackoverflow_17(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![17, -32, 99]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![17, -32, 99]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_concat1(in1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_18 无法实现，维度已经超过二维

// stackoverflow_19 无法实现，多次重复出现同一表达式，且有数组切片运算

// stackoverflow_20
// 原输入超过4行，删除最后一行，将小数修正为整数
fn stackoverflow_20(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![7, 2, 1]);
    input1.push(vec![4, 5, 1]);
    input1.push(vec![4, 4, 2]);
    input1.push(vec![3, 4, 3]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![3]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_argmax(in1);
    let o2 = builder.tf_one_hot(o1, in2);
    let _ = builder.tf_cast(o2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_21 tf.gather

// stackoverflow_22
// 原本第二个输入是小数，手动改成整数
fn stackoverflow_22(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![3, 1, 0]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![6, 4]);
    input2.push(vec![5, 10]);
    input2.push(vec![3, 4]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_cast(in1);
    let _ = builder.tf_tensordot(o1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_23 无法实现，维度超过二维

// stackoverflow_24
// 原本输入是小数，手动改成整数
fn stackoverflow_24(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![30, 10, 40, 50, 20, 80, -60, -70]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![5, 0, -20, 0, 10, -10, 0, 20]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![30, 10, 40, 50, 20, 80, -60, -70]);

    let mut input4 : Vec<Vec<i64>> = Vec::new();
    input4.push(vec![5, 0, -20, 0, 10, -10, 0, 20]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);
    let in4 = builder.var(input4);

    let o1 = builder.tf_cast(in2);
    let o2 = builder.tf_divide(in1, in4);
    let _ = builder.tf_where3(o1, o2, in3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_25 tf.tile

// stackoverflow_26 
// 由于原输入是三维的，手动实现第一次reducesum
fn stackoverflow_26(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![6, 4]);
    input1.push(vec![3, -7]);
    input1.push(vec![30, 3]);

    let in1 = builder.var(input1);

    let _ = builder.tf_reduce_sum1(in1);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// stackoverflow_27
fn stackoverflow_27(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![0, 3, 5, 6]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![8]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_one_hot(in1, in2);
    let o2 = builder.tf_reduce_max0(o1);
    let _ = builder.tf_cast(o2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// stackoverflow_28 无法实现，维度超过二维

//todo stackoverflow_29 tf.searchsorted

// stackoverflow_30 无法实现，维度超过二维

// stackoverflow_31
// 手动将稀疏张量变为普通张量
// 将浮点数变为整数
fn stackoverflow_31(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
     
    let mut input1 : Vec<Vec<i64>> = Vec::new();   
    input1.push(vec![10, 15]);
    input1.push(vec![0, -20]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();   
    input2.push(vec![30, 10]);
    input2.push(vec![2, -10]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_subtract(in2, in1);
    let o2 = builder.tf_square(o1);
    let _ = builder.tf_reduce_sum(o2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library); 
}

// stackoverflow_32
// 由于tensordot第二个参数方向是纵轴方向，所以自己手动用expand_dims调整，由于是小数，调整为整数
fn stackoverflow_32(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 6, 2, 1]);
    input1.push(vec![3, 1, 4, 2]);
    input1.push(vec![2, 1, 2, 5]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![0]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![4]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_range(in2, in3);
    let o2 = builder.tf_cast(o1);
    let o3 = builder.tf_expand_dims(o2);
    let _ = builder.tf_tensordot(in1, o3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_33 无法实现，数组超过二维

// stackoverflow_34
// 原本第一个输入不是二维数组，手动选取二维数组，[[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[10, 20], [30, 40]]]为[[1, 2]], [[5, 6]], [[10, 20]]
fn stackoverflow_34(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 2]);
    input1.push(vec![5, 6]);
    input1.push(vec![10, 20]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![3, 5, 10]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let _ = builder.tf_tensordot(in2, in1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

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

    let mut input4 : Vec<Vec<i64>> = Vec::new();
    input4.push(vec![9, 8]);
    input4.push(vec![7, 6]);
    input4.push(vec![5, 4]);
    input4.push(vec![90, 80]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);
    let in4 = builder.var(input4);

    let o1 = builder.tf_expand_dims(in3);
    let o2 = builder.tf_subtract(in1, in2);
    let o3 = builder.tf_multiply(o1, o2);
    let _ = builder.tf_add(in4, o3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_36
fn stackoverflow_36(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 0, 1, 1, 0, 1, 0, 1]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![0]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![8]);

    let mut input4 : Vec<Vec<i64>> = Vec::new();
    input4.push(vec![1, 0, 1, 1, 0, 1, 0, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);
    let in4 = builder.var(input4);

    let o1 = builder.tf_range(in2, in3);
    let o2 = builder.tf_add(in1, o1);
    let o3 = builder.tf_divide(in4, o2);
    let _ = builder.tf_cast(o3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_37
// 第一个输入不是二维的，手动修改[[[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], [[1.2, 3.4, 5.6], [7.8, 9.8, 7.6]]]],为[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]，[1.2, 3.4, 5.6], [7.8, 9.8, 7.6]]
// 输入是小数，手动修改为整数
// 由于tensordot第二个参数方向是纵轴方向，所以自己手动用expand_dims调整，由于是小数，调整为整数
fn stackoverflow_37(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 2, 3]);
    input1.push(vec![4, 5, 6]);
    input1.push(vec![12, 34, 56]);
    input1.push(vec![78, 98, 76]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![5, 10, 20]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_expand_dims(in2);
    let _ = builder.tf_tensordot(in1, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_38
fn stackoverflow_38(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![9, 2, 5, 3, 7, 4]);
    input1.push(vec![9, 2, 5, 3, 7, 4]);
    input1.push(vec![9, 2, 5, 3, 7, 4]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![0, 0, 1, 0, 1, 0]);
    input2.push(vec![1, 0, 0, 0, 0, 0]);
    input2.push(vec![0, 1, 1, 1, 0, 1]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![0, 0, 1, 0, 1, 0]);
    input3.push(vec![1, 0, 0, 0, 0, 0]);
    input3.push(vec![0, 1, 1, 1, 0, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_reduce_max(in3);
    let o2 = builder.tf_multiply(in1, in2);
    let o3 = builder.tf_maximum(o1, o2);
    let _ = builder.tf_reduce_prod(o3);

    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

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

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![-15, 1, 0, 2]);
    input2.push(vec![1, 0, 0, 0]);
    input2.push(vec![-1, 0, -11, 25]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_cast(in1);
    let o2 = builder.tf_square(o1);
    let _ = builder.tf_multiply(o2, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_40 tf.sparse.to_dense tf.SparseTensor

// stackoverflow_41
fn stackoverflow_41(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![5, 2, 8, 2, 4, 1, 1, 0, 2, 1]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![0]);

    let mut input4 : Vec<Vec<i64>> = Vec::new();
    input4.push(vec![10]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);
    let in4 = builder.var(input4);

    let o1 = builder.tf_constant(in2);
    let o2 = builder.tf_range(in3, in4);
    let o3 = builder.tf_not_equal(o1, o2);
    let _ = builder.tf_boolean_mask(in1, o3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_42 
fn stackoverflow_42(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![4, 6, 2, 6, 7, 3, -3]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![4, 6, 2, 6, 7, 3, -3]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_reduce_max(in1);
    let o2 = builder.tf_equal(in2, o1);
    let _ = builder.tf_cast(o2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo stackoverflow_43 tf.gather_nd

// stackoverflow_44 无法实现，维度超过二维

// stackoverflow_45
// 原本的输入是三维的，为了适应4*10的数组
// [[[12, 34], [56, 78], [23, 54], [76, 78], [42, 24]]]改为[[12, 34], [56, 78], [23, 54], [76, 78]]， [1, 0, 1, 0, 1]改为[1, 0, 1, 0]
fn stackoverflow_45(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![12, 34]);
    input1.push(vec![56, 78]);
    input1.push(vec![23, 54]);
    input1.push(vec![76, 78]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1, 0, 1, 0]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![12, 34]);
    input3.push(vec![56, 78]);
    input3.push(vec![23, 54]);
    input3.push(vec![76, 78]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_sequence_mask(in2);
    let o2 = builder.tf_roll(in1);
    let _ = builder.tf_where3(o1, o2, in3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_46
// 输入超出范围，[3, 4, 1]改成[1, 2]
fn stackoverflow_46(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 2]);

    let in1 = builder.var(input1);

    let o1 = builder.tf_sequence_mask(in1);
    let o2 = builder.tf_where1(o1);
    let _ = builder.tf_cast(o2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// stackoverflow_47 无法实现，出现重复的表达式和其他运算

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

// stackoverflow_49 无法实现，首先是形状不对，其次是无法同时用一个方法两次

// stackoverflow_50
// 原输入有5行，修改为4行，手动调整python的数组格式(5,)为[1, 5]
fn stackoverflow_50(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 4]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![6]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![3]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_fill(in1, in3);
    let o2 = builder.tf_one_hot(o1, in2);
    let _ = builder.tf_cast(o2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// autopandas_benchmarks

//todo autopandas1 tf.gather

// autopandas2 数组下标取元素，不是tf的表达式

//todo autopandas3 tf.reshape

// autopandas4
// 原输入超出4*10范围，[[1, 2, 3, 4, 5], [9, 8, 7, 6, 5], [3, 0, 2, 5, 8], [8, 8, 6, 3, 2], [2, 0, 7, 7, 3], [9, 0, 3, 2, 7], [1, 3, 8, 9, 4]]改为
// [[1, 2, 3, 4, 5], [9, 8, 7, 6, 5], [3, 0, 2, 5, 8], [8, 8, 6, 3, 2]]，并手动实现其中的取下标
fn autopandas4(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 2, 3, 4, 5]);
    input1.push(vec![9, 8, 7, 6, 5]);
    input1.push(vec![3, 0, 2, 5, 8]);
    input1.push(vec![8, 8, 6, 3, 2]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![1, 1, 0, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_cast(in2);
    let _ = builder.tf_boolean_mask_(in1, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

//todo autopandas5 tf.gather tf.argsort

//todo autopandas6 tf.reshape

//todo autopandas7 tf.gather tf.argsort

// autopandas8
// 原输入超出4*10范围，[[5, 7], [6, 8], [-1, 9], [-2, 10], [2, 11], [1, 12], [3, -3]]改为[[5, 7], [6, 8], [-1, 9], [-2, 10]]，并手动实现其中的取下标
fn autopandas8(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![5, 7]);
    input1.push(vec![6, 8]);
    input1.push(vec![-1, 9]);
    input1.push(vec![-2, 10]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![5, 6, -1, -2]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![1, 1, 1, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);

    let o1 = builder.tf_greater(in2, in3);
    let _ = builder.tf_boolean_mask_(in1, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// autopandas9 无法实现，重复的表达式出现多次

// autopandas10
// 手动实现tf.math.logical_not(tf.math.is_nan(in1))，为[0, 1, 1, 0, 1, 1]
// [float('nan'), 11, 12, float('nan'), 16, 18]改为[-1, 11, 12, -1, 16, 18]
fn autopandas10(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![-1, 11, 12, -1, 16, 18]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![0, 1, 1, 0, 1, 1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    let o1 = builder.tf_boolean_mask(in1, in2);
    let _ = builder.tf_cast(o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// autopandas11
// 暂时还没实现expand_dims中axis=0的实现，所以先用个中间结果保持住
fn autopandas11(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 4, 2, 7, 6]);
    input1.push(vec![20, 10, 50, 40, 30]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![0, 1, 2, 3, 4]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    
    let o1 = builder.tf_concat0(in2, in1);
    let _ = builder.tf_transpose(o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// autopandas12 无法实现，超出二维

// autopandas13
// 输入超出4*10范围，[[101, 0, 11, 0], [102, 1, 12, 4], [103, 2, 13, 2], [104, 3, 14, 8], [105, 4, 15, 4],
// [106, 5, 16, 5], [107, 6, 17, 4], [108, 7, 18, 7], [109, 8, 19, 7], [110, 9, 20, 4]]修改为
// [[103, 2, 13, 2], [105, 4, 15, 4], [107, 6, 17, 4], [108, 7, 18, 7]]
// 手动实现[:, 1]切片
fn autopandas13(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![103, 2, 13, 2]);
    input1.push(vec![105, 4, 15, 4]);
    input1.push(vec![107, 6, 17, 4]);
    input1.push(vec![108, 7, 18, 7]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![2, 4, 6]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![2, 4, 6, 7]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);
    
    let o1 = builder.tf_expand_dims(in2);
    let o2 = builder.tf_equal(in3, o1);
    let o3 = builder.tf_reduce_any0(o2);
    let _ = builder.tf_boolean_mask_(in1, o3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// autopandas14
// 原数据是float(nan)，自己改成-1
fn autopandas14(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 0, 1, 2]);
    input1.push(vec![1, 1, 3, 4]);
    input1.push(vec![2, 0, 1, 2]);
    input1.push(vec![2, 1, 3, 4]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![4, 1]);

    let mut input3 : Vec<Vec<i64>> = Vec::new();
    input3.push(vec![-1]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    let in3 = builder.var(input3);
    
    let o1 = builder.tf_fill(in2, in3);
    let o2 = builder.tf_cast(in1);
    let _ = builder.tf_concat1(o2, o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// autopandas15
fn autopandas15(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1, 1, 2, 1, 3, 2]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![0]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);
    
    let _ = builder.tf_cumsum(in1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// autopandas16
fn autopandas16(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();

    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![0, 6, 0]);
    input1.push(vec![3, 101, 14]);
    input1.push(vec![0, 91, 6]);
    input1.push(vec![5, 15, 0]);

    let in1 = builder.var(input1);
    
    let o1 = builder.tf_cast(in1);
    let _ = builder.tf_reduce_mean(o1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}