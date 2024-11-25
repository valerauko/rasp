extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate rasp_runtime as runtime;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::support::{load_library_permanently};

use std::fs;
use std::path::{Path};

mod read;

use runtime::{Primitive};

struct Namespace<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    engine: ExecutionEngine<'ctx>,
}

type MulFn = unsafe extern "C" fn(i64, i64) -> i64;

impl<'ctx> Namespace<'ctx> {
    fn compile(&self, item: Primitive) {
        // match item {
        //     Primitive::List(lst) => {
        //         match lst.first() {
        //             Some(item) => match item {
        //                 Primitive::Symbol(sym) => {
        //                     match sym.as_str() {
        //                         "defn" => unreachable!(),
        //                         _ => match self.module.get_function(&sym) {
        //                             Some(fun) => {
        //                                 let ret = self.builder.build_call(
        //                                     fun,
        //                                     &[
        //                                         lst.drop_first().unwrap().first().unwrap().into(),
        //                                         lst.drop_first().unwrap().drop_first().unwrap().first().unwrap().into(),
        //                                     ],
        //                                     sym
        //                                 ).unwrap().try_as_basic_value().left();
        //                                 println!("{:#?}", ret);
        //                             }
        //                             None => panic!("Unknown symbol: {}", sym),
        //                         }
        //                     }
        //                 }
        //                 _ => panic!("First element of a call should be a symbol"),
        //             }
        //             None => todo!()
        //         }
        //     }
        //     _ => todo!()
        // };
    }

    fn add_mul(&self) -> Option<JitFunction<MulFn>> {
        let i64_t = self.context.i64_type();
        // TODO: variadic *
        let fun_t = i64_t.fn_type(&[i64_t.into(), i64_t.into()], false);
        let fun = self.module.add_function("*", fun_t, None);
        let block = self.context.append_basic_block(fun, "entry");

        self.builder.position_at_end(block);

        let x = fun.get_nth_param(0)?.into_int_value();
        let y = fun.get_nth_param(1)?.into_int_value();

        let ret = self.builder.build_int_mul(x, y, "*").unwrap();
        self.builder.build_return(Some(&ret)).unwrap();

        unsafe { self.engine.get_function("*").ok() }
    }
}

fn main() {
    let testfile = fs::read_to_string("resources/macro.ras").unwrap();
    let parsed = read::parse(&testfile);

    let context = Context::create();
    let module = context.create_module("foo");
    let engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    let ns = Namespace {
        context: &context,
        module,
        builder: context.create_builder(),
        engine,
    };

    load_library_permanently(Path::new("../runtime/target/debug/librasp_runtime.so"));
    

    let mul = ns.add_mul().unwrap();

    unsafe {
        println!("{:?}", mul.call(2, 4));
    }
}
