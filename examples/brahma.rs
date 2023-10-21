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
            "mytest1".to_string(),
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
        mytest1,
        // mytest2,
        // mytest3,
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
    #[structopt(short = "c", long = "synthesize-constants")]
    synthesize_constants: bool,

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

/*
下一步的重点：所有的样例是按照单个输入处理的，比如abs里面只接受一个数（Id），而不允许接受一个数组（vec！【Id】）
例如，abs（a）是可以的，但是abs（【a， b， c】）就没办法
之前做的工作，也只是在外部包装了一下数组，即定义三个var，然后push到数组里面，然后遍历数组，传入单个参数进行处理
这种工作对于需要数组操作的方法来说无能为力，因为它没有办法体现在最终结果里面
所以能够看到，在operator方法里面只有几个tf方法，因为这些太多的tf表达式和数组有关了，然而目前还没有办法实现数组的操作
之前想到的一种方法是，可以考虑把这个过程的输出插入到最终输出的过程中，来达到目的
比如mytest2里面的样例，可以把expanddim这个方法的输出插入到add的输出之前，但是目前做不到显示expanddim这个步骤
而且，根据结果可知，如果不把他们放到数组里面的话，最后的结果会被“合并”
比如【a， b， c】和【d， e， f】相加应该是a+d， b+e， c+f，但是输出只显示一个，这是不对的
所以当务之急还是如何将代码由处理单个变量扩充到处理数组
当然目前也不是一无进展，处理表达式第一步先要在operator里面定义，然后加入到lib里面的component，在component.rs里面编写，再在builder里面添加函数，这样就可以了
其余的都简单，直接改成数组即可，唯独component.rs里面的make_operator和make_expression需要大改，估计lib也得改，所以是个大工程o(╥﹏╥)o
*/

// 注释中写入每个样例的名字，方便查找，所以函数命名就随意了

/* test_add
  examples = [
      benchmark.Example(
          inputs=[
              [10],
              [20],
          ],
          output=[30],
      ),
  ]
  constants = [0]
  description = 'Add elementwise'
  target_program = 'tf.add(in1, in2)'
  source = 'test'
*/

fn mytest1(context: &z3::Context, opts: &Options) -> SynthResult<Program> {

    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    
    // Modify：调用var()的时候接收一个参数，将输入的vec传入到spec中
    
    let mut input1 : Vec<Vec<i64>> = Vec::new();
    input1.push(vec![1,-1,1]);

    let mut input2 : Vec<Vec<i64>> = Vec::new();
    input2.push(vec![2,-3,2]);

    let in1 = builder.var(input1);
    let in2 = builder.var(input2);

    

    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    return synthesize(opts, context, &spec, &library);
}

// TODO：严格来说应该是const的长度和值都要定义，但是这里默认长度就是值，看看后续能不能改

/* simple_using_constant
  examples = [
      benchmark.Example(
          inputs=[
              [1, 2, 3],
          ],
          output=[101, 102, 103]
      ),
  ]
  constants = [100]
  description = 'Add 100 to every element'
  target_program = 'tf.add(in1, tf.constant(100))'
  source = 'handwritten task'
*/

/*fn mytest2(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let mut library = Library::brahma_std();
    let sz = 3;
    library
        .components
        .push(component::const_(if opts.synthesize_constants {
            None
        } else {
            Some(sz)
        }));
    let mut builder = ProgramBuilder::new();
    let in1 = builder.var();
    let const100 = builder.const_(sz);
    let _ = builder.tf_add(in1, const100);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library, sz as u32)
}

/* google_10
  examples = [
      benchmark.Example(
          inputs=[
              [10, 20, 0, 40, 0, 30],
              [1, 1, 0, 1, 0, 1],
          ],
          output=[10, 20, 40, 30]
      ),
  ]
  constants = []
  description = 'gather the marked elements'
  target_program = 'tf.boolean_mask(in1, tf.cast(in2, tf.bool))'
  source = ('Proposed by Googler at an internal demo on 8/13/2019, '
            'simplified slightly')
*/

fn mytest3(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    let in1 = builder.var();
    let in2 = builder.var();
    let _ = builder.tf_boolean_mask(in1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library, 6)
}

/* simple_cast
  examples = [
      benchmark.Example(
          inputs=[
              [12, 34, 56]
          ],
          output=[12.0, 34.0, 56.0],
      ),
  ]
  constants = []
  description = 'Cast an int tensor into a float tensor'
  target_program = 'tf.cast(in1, tf.float32)'
  source = 'handwritten task' 
*/

fn mytest4(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    let in1 = builder.var();
    let _ = builder.tf_cast(in1);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library, 3)
}

/* simple_using_primitive_input
  examples = [
      benchmark.Example(
          inputs=[
              123,
              tf.constant(45),
          ],
          output=tf.constant(168),
      ),
  ]
  constants = []
  description = 'Add primitive int and scalar int tensor'
  target_program = 'tf.add(in2, tf.constant(in1))'
  source = 'handwritten task'
 */

 fn mytest5(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let mut library = Library::brahma_std();
    let sz = 1;
    library
        .components
        .push(component::const_(if opts.synthesize_constants {
            None
        } else {
            Some(sz)
        }));
    let mut builder = ProgramBuilder::new();
    let in1 = builder.var();
    let in2 = builder.const_(sz);
    let _ = builder.tf_add(in1, in2);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library, 1)
}*/

