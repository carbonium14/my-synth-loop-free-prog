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
    // if opts.only_fast {
    //     opts.problems = vec![
    //         "p1".to_string(),
    //         "p2".to_string(),
    //         "p3".to_string(),
    //         "p4".to_string(),
    //         "p5".to_string(),
    //         "p6".to_string(),
    //         "p7".to_string(),
    //         "p10".to_string(),
    //         "p18".to_string(),
    //     ];
    // }
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
        // p1,
        // p2,
        // p3,
        // p4,
        // p5,
        // p6,
        // p7,
        // p8,
        // p9,
        // p10,
        // p11,
        // p12,
        // p13,
        // p14,
        // p15,
        // p16,
        // p17,
        // p18,
        // p19,
        // p20,
        // p21,
        // p22,
        // p23,
        // p24,
        // p25,
        mytest1,
        mytest2,
        mytest3,
        mytest4,
        mytest5,
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
    library: &Library,
) -> SynthResult<Program> {
    Synthesizer::new(context, library, spec)?
        .set_timeout(opts.timeout)
        .should_synthesize_minimal_programs(opts.minimal)
        .synthesize()
}

fn mytest1(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    // 如果不需要常数，那么就去掉上面的mut，否则的加上mut来符合rust语法
    // 注意，这里是设置常数的，如果常数是0可以不用写，但是其余的值得写
    // 看起来非零常数只能声明一次，因为它只有一次声明
    // library
    //     .components
    //     .push(component::const_(if opts.synthesize_constants {
    //         None
    //     } else {
    //         Some(1)
    //     }));
    // 注意，不能没有var，而且var一定要在最前面！！！
    let mut builder = ProgramBuilder::new();
    let a = builder.var();
    let b = builder.var();
    let _ = builder.tf_add(a, b);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

fn mytest2(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    let a00 = builder.var();
    let a01 = builder.var();
    let a02 = builder.var();
    let a10 = builder.var();
    let a11 = builder.var();
    let a12 = builder.var();
    let _ = builder.tf_add(a00, a10);
    let _ = builder.tf_add(a01, a11);
    let _ = builder.tf_add(a02, a12);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

fn mytest3(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let mut library = Library::brahma_std();
    library
        .components
        .push(component::const_(if opts.synthesize_constants {
            None
        } else {
            Some(7)
        }));
    let mut builder = ProgramBuilder::new();
    let a00 = builder.var();
    let a11 = builder.var();
    let a22 = builder.var();
    let a33 = builder.var();
    let a44 = builder.var();
    let tfconst = builder.const_(7);
    let _a01 = builder.const_(0);
    let _a02 = builder.const_(0);
    let _a03 = builder.const_(0);
    let _a04 = builder.const_(0);
    let _a10 = builder.const_(0);
    let _a12 = builder.const_(0);
    let _a13 = builder.const_(0);
    let _a14 = builder.const_(0);
    let _a20 = builder.const_(0);
    let _a21 = builder.const_(0);
    let _a23 = builder.const_(0);
    let _a24 = builder.const_(0);
    let _a30 = builder.const_(0);
    let _a31 = builder.const_(0);
    let _a32 = builder.const_(0);
    let _a34 = builder.const_(0);
    let _a40 = builder.const_(0);
    let _a41 = builder.const_(0);
    let _a42 = builder.const_(0);
    let _a43 = builder.const_(0);
    let _ = builder.tf_mul(a00, tfconst);
    let _ = builder.tf_mul(a11, tfconst);
    let _ = builder.tf_mul(a22, tfconst);
    let _ = builder.tf_mul(a33, tfconst);
    let _ = builder.tf_mul(a44, tfconst);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

fn mytest4(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let library = Library::brahma_std();
    let mut builder = ProgramBuilder::new();
    let a00 = builder.var();
    let a01 = builder.var();
    let a02 = builder.var();
    let a03 = builder.var();
    let a10 = builder.var();
    let a11 = builder.var();
    let a12 = builder.var();
    let a13 = builder.var();
    let a20 = builder.var();
    let a21 = builder.var();
    let a22 = builder.var();
    let a23 = builder.var();
    let o10 = builder.tf_add(a00, a01);
    let o11 = builder.tf_add(o10, a02);
    let row1 = builder.tf_add(o11, a03);
    let o20 = builder.tf_add(a10, a11);
    let o21 = builder.tf_add(o20, a12);
    let row2 = builder.tf_add(o21, a13);
    let o30 = builder.tf_add(a20, a21);
    let o31 = builder.tf_add(o30, a22);
    let row3 = builder.tf_add(o31, a23);
    let _ = builder.tf_div(a00, row1);
    let _ = builder.tf_div(a01, row1);
    let _ = builder.tf_div(a02, row1);
    let _ = builder.tf_div(a03, row1);
    let _ = builder.tf_div(a10, row2);
    let _ = builder.tf_div(a11, row2);
    let _ = builder.tf_div(a12, row2);
    let _ = builder.tf_div(a13, row2);
    let _ = builder.tf_div(a20, row3);
    let _ = builder.tf_div(a21, row3);
    let _ = builder.tf_div(a22, row3);
    let _ = builder.tf_div(a23, row3);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

fn mytest5(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
    let mut library = Library::brahma_std();
    library
        .components
        .push(component::const_(if opts.synthesize_constants {
            None
        } else {
            Some(1)
        }));
    let mut builder = ProgramBuilder::new();
    let a0 = builder.var();
    let a1 = builder.var();
    let a2 = builder.var();
    let a3 = builder.var();
    let a4 = builder.var();
    let a5 = builder.var();
    let b0 = builder.const_(1);
    let b1 = builder.const_(1);
    let b2 = builder.const_(0);
    let b3 = builder.const_(1);
    let b4 = builder.const_(0);
    let b5 = builder.const_(1);
    let _ = builder.tf_boolean_mask(a0, b0);
    let _ = builder.tf_boolean_mask(a1, b1);
    let _ = builder.tf_boolean_mask(a2, b2);
    let _ = builder.tf_boolean_mask(a3, b3);
    let _ = builder.tf_boolean_mask(a4, b4);
    let _ = builder.tf_boolean_mask(a5, b5);
    let spec = builder.finish();

    synthesize(opts, context, &spec, &library)
}

// fn p1(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let a = builder.var();
//     let b = builder.const_(1);
//     let c = builder.sub(a, b);
//     let _ = builder.and(a, c);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p2(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let a = builder.var();
//     let b = builder.const_(1);
//     let c = builder.add(a, b);
//     let _ = builder.and(a, c);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p3(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library::brahma_std();

//     let mut builder = ProgramBuilder::new();
//     let a = builder.var();
//     let b = builder.const_(0);
//     let c = builder.sub(b, a);
//     let _ = builder.and(a, c);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p4(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let a = builder.var();
//     let b = builder.const_(1);
//     let c = builder.sub(a, b);
//     let _ = builder.xor(a, c);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p5(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let a = builder.var();
//     let b = builder.const_(1);
//     let c = builder.sub(a, b);
//     let _ = builder.or(a, c);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p6(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let a = builder.var();
//     let b = builder.const_(1);
//     let c = builder.add(a, b);
//     let _ = builder.or(a, c);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p7(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(std::u64::MAX)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     // o1 = bvnot(x) = xor(x, MAX)
//     let a = builder.const_(std::u64::MAX);
//     let o1 = builder.xor(x, a);
//     // o2 = bvadd(x, 1)
//     let b = builder.const_(1);
//     let o2 = builder.add(x, b);
//     let _ = builder.and(o1, o2);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p8(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let a = builder.var();
//     let b = builder.const_(1);
//     let c = builder.sub(a, b);
//     let d = builder.const_(std::u64::MAX);
//     let e = builder.xor(a, d);
//     let _ = builder.and(c, e);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p9(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(31)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let a = builder.var();
//     let b = builder.const_(31);
//     let c = builder.shr_u(a, b);
//     let d = builder.xor(a, c);
//     let _ = builder.sub(d, c);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p10(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library::brahma_std();

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let y = builder.var();
//     let a = builder.and(x, y);
//     let b = builder.xor(x, y);
//     let _ = builder.le_u(b, a);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p11(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library::brahma_std();

//     let mut builder = ProgramBuilder::new();

//     let x = builder.var();
//     let y = builder.var();

//     // not y
//     let a = builder.const_(std::u64::MAX);
//     let b = builder.xor(a, y);

//     let c = builder.and(x, b);
//     let _ = builder.gt_u(c, y);

//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p12(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library::brahma_std();

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let y = builder.var();

//     // not y
//     let a = builder.const_(std::u64::MAX);
//     let b = builder.xor(a, y);

//     let c = builder.and(x, b);
//     let _ = builder.le_u(c, y);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p13(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(31)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();

//     let a = builder.const_(31);
//     let o1 = builder.shr_s(x, a);

//     // neg(x) = 0 - x
//     let b = builder.const_(0);
//     let o2 = builder.sub(b, x);

//     let o3 = builder.shr_u(o2, a);
//     let _ = builder.or(o1, o3);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p14(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let y = builder.var();
//     let o1 = builder.and(x, y);
//     let o2 = builder.xor(x, y);
//     let a = builder.const_(1);
//     let o3 = builder.shr_u(o2, a);
//     let _ = builder.add(o1, o3);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p15(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let y = builder.var();
//     let o1 = builder.or(x, y);
//     let o2 = builder.xor(x, y);
//     let a = builder.const_(1);
//     let o3 = builder.shr_u(o2, a);
//     let _ = builder.sub(o1, o3);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p16(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library::brahma_std();

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let y = builder.var();
//     let o1 = builder.xor(x, y);
//     let a = builder.ge_u(x, y);

//     // o2 = neg(a)
//     let b = builder.const_(0);
//     let o2 = builder.sub(b, a);

//     let o3 = builder.and(o1, o2);
//     let _ = builder.xor(o3, y);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p17(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let mut library = Library::brahma_std();
//     library
//         .components
//         .push(component::const_(if opts.synthesize_constants {
//             None
//         } else {
//             Some(1)
//         }));

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let a = builder.const_(1);
//     let o1 = builder.sub(x, a);
//     let o2 = builder.or(x, o1);
//     let o3 = builder.add(o2, a);
//     let _ = builder.and(o3, x);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// // The `brahma_std` library should be good for `p <= 17`. From here on out, we
// // require custom libraries.

// fn p18(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     // In the paper, redor(x) seems to produce booleans, which have their own
//     // operations. Turns out if we use zero and non-zero as 32-bit wide false
//     // and true, we can solve the whole thing with our `brahma_std` library.
//     let library = Library::brahma_std();

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let a = builder.const_(1);
//     let o1 = builder.sub(x, a);
//     let o2 = builder.and(o1, x);

//     // o3 = redor(x) = "reduce bits with or"
//     //    = 1 if any bit is set, otherwise 0
//     //    = `gt_u x, 0`
//     let b = builder.const_(0);
//     let o3 = builder.gt_u(x, b);

//     // o4 = redor(o2)
//     let o4 = builder.gt_u(o2, b);

//     // o5 = bool-not(o4)
//     let o5 = builder.select(o4, b, a);

//     // res = bool-and(o5, o3)
//     let _ = builder.and(o5, o3);

//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p19(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library {
//         components: vec![
//             component::shr_u(),
//             component::xor(),
//             component::and(),
//             component::shl(),
//             component::xor(),
//             component::xor(),
//         ],
//     };

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let m = builder.var();
//     let k = builder.var();
//     let o1 = builder.shr_u(x, k);
//     let o2 = builder.xor(x, o1);
//     let o3 = builder.and(o2, m);
//     let o4 = builder.shl(o3, k);
//     let o5 = builder.xor(o4, o3);
//     let _ = builder.xor(o5, x);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p20(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library {
//         components: vec![
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(0)
//             }),
//             component::sub(),
//             component::and(),
//             component::add(),
//             component::xor(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(2)
//             }),
//             component::shr_u(),
//             component::div_u(),
//             component::or(),
//         ],
//     };

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();

//     // o1 = neg(x)
//     let a = builder.const_(0);
//     let o1 = builder.sub(a, x);

//     let o2 = builder.and(x, o1);
//     let o3 = builder.add(x, o2);
//     let o4 = builder.xor(x, o2);
//     let b = builder.const_(2);
//     let o5 = builder.shr_u(o4, b);
//     let o6 = builder.div_u(o5, o2);
//     let _ = builder.or(o6, o3);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p21(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library {
//         components: vec![
//             component::eq(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(0)
//             }),
//             component::sub(),
//             component::xor(),
//             component::eq(),
//             component::sub(),
//             component::xor(),
//             component::and(),
//             component::and(),
//             component::xor(),
//             component::xor(),
//         ],
//     };

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let a = builder.var();
//     let b = builder.var();
//     let c = builder.var();

//     // o1 = neg(eq(x, c))
//     let d = builder.eq(x, c);
//     let e = builder.const_(0);
//     let o1 = builder.sub(e, d);

//     let o2 = builder.xor(a, c);

//     // o3 = neg(eq(x, a))
//     let f = builder.eq(x, a);
//     let o3 = builder.sub(e, f);

//     let o4 = builder.xor(b, c);
//     let o5 = builder.and(o1, o2);
//     let o6 = builder.and(o3, o4);
//     let o7 = builder.xor(o5, o6);
//     let _ = builder.xor(o7, c);

//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p22(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library {
//         components: vec![
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(1)
//             }),
//             component::shr_u(),
//             component::xor(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(2)
//             }),
//             component::shr_u(),
//             component::xor(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(0x11111111)
//             }),
//             component::and(),
//             component::mul(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(28)
//             }),
//             component::shr_u(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(1)
//             }),
//             component::and(),
//         ],
//     };

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let a = builder.const_(1);
//     let o1 = builder.shr_u(x, a);
//     let o2 = builder.xor(o1, x);
//     let b = builder.const_(2);
//     let o3 = builder.shr_u(o2, b);
//     let o4 = builder.xor(o2, o3);
//     let c = builder.const_(0x11111111);
//     let o5 = builder.and(o4, c);
//     let o6 = builder.mul(o5, c);
//     let d = builder.const_(28);
//     let o7 = builder.shr_u(o6, d);
//     let e = builder.const_(1);
//     let _ = builder.and(o7, e);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p23(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library {
//         components: vec![
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(1)
//             }),
//             component::shr_u(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(0x55555555)
//             }),
//             component::and(),
//             component::sub(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(0x33333333)
//             }),
//             component::and(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(2)
//             }),
//             component::shr_u(),
//             component::and(),
//             component::add(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(4)
//             }),
//             component::add(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(0x0F0F0F0F)
//             }),
//             component::and(),
//         ],
//     };

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let a = builder.const_(1);
//     let o1 = builder.shr_u(x, a);
//     let b = builder.const_(0x55555555);
//     let o2 = builder.and(o1, b);
//     let o3 = builder.sub(x, o2);
//     let c = builder.const_(0x33333333);
//     let o4 = builder.and(o3, c);
//     let d = builder.const_(2);
//     let o5 = builder.shr_u(o3, d);
//     let o6 = builder.and(o5, c);
//     let o7 = builder.add(o4, o6);
//     let e = builder.const_(4);
//     let o8 = builder.shr_u(o7, e);
//     let o9 = builder.add(o8, o7);
//     let f = builder.const_(0x0F0F0F0F);
//     let _ = builder.and(o9, f);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p24(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library {
//         components: vec![
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(1)
//             }),
//             component::sub(),
//             component::shr_u(),
//             component::or(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(2)
//             }),
//             component::shr_u(),
//             component::or(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(4)
//             }),
//             component::shr_u(),
//             component::or(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(8)
//             }),
//             component::shr_u(),
//             component::or(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(16)
//             }),
//             component::shr_u(),
//             component::or(),
//             component::add(),
//         ],
//     };

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let a = builder.const_(1);
//     let o1 = builder.sub(x, a);
//     let o2 = builder.shr_u(o1, a);
//     let o3 = builder.or(o1, o2);
//     let b = builder.const_(2);
//     let o4 = builder.shr_u(o3, b);
//     let o5 = builder.or(o3, o4);
//     let c = builder.const_(4);
//     let o6 = builder.shr_u(o5, c);
//     let o7 = builder.or(o5, o6);
//     let d = builder.const_(8);
//     let o8 = builder.shr_u(o7, d);
//     let o9 = builder.or(o7, o8);
//     let e = builder.const_(16);
//     let o10 = builder.shr_u(o9, e);
//     let o11 = builder.or(o9, o10);
//     let _ = builder.add(o11, a);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }

// fn p25(context: &z3::Context, opts: &Options) -> SynthResult<Program> {
//     let library = Library {
//         components: vec![
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(0xFFFF)
//             }),
//             component::and(),
//             component::const_(if opts.synthesize_constants {
//                 None
//             } else {
//                 Some(16)
//             }),
//             component::shr_u(),
//             component::and(),
//             component::shr_u(),
//             component::mul(),
//             component::mul(),
//             component::mul(),
//             component::mul(),
//             component::shr_u(),
//             component::add(),
//             component::and(),
//             component::shr_u(),
//             component::add(),
//             component::shr_u(),
//             component::add(),
//             component::add(),
//         ],
//     };

//     let mut builder = ProgramBuilder::new();
//     let x = builder.var();
//     let y = builder.var();
//     let a = builder.const_(0xFFFF);
//     let o1 = builder.and(x, a);
//     let b = builder.const_(16);
//     let o2 = builder.shr_u(x, b);
//     let o3 = builder.and(y, a);
//     let o4 = builder.shr_u(y, b);
//     let o5 = builder.mul(o1, o3);
//     let o6 = builder.mul(o2, o3);
//     let o7 = builder.mul(o1, o4);
//     let o8 = builder.mul(o2, o4);
//     let o9 = builder.shr_u(o5, b);
//     let o10 = builder.add(o6, o9);
//     let o11 = builder.and(o10, a);
//     let o12 = builder.shr_u(o10, b);
//     let o13 = builder.add(o7, o11);
//     let o14 = builder.shr_u(o13, b);
//     let o15 = builder.add(o14, o12);
//     let _ = builder.add(o15, o8);
//     let spec = builder.finish();

//     synthesize(opts, context, &spec, &library)
// }
