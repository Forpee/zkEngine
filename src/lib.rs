extern crate wasmi;

use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

#[wasm_bindgen]
pub fn read_files(wasm_mod: &[u8], f_name: &str, args: Vec<i32>) -> String {
    let test_phantom_vec: Vec<String> = Vec::new();
    let tracer = wasmi::tracer::Tracer::new(HashMap::default(), &test_phantom_vec);

    // Load wasm binary and prepare it for instantiation.
    let module = wasmi::Module::from_buffer(&wasm_mod).unwrap();

    // Instantiate a module with empty imports and
    // asserting that there is no `start` function.
    let tracer = Rc::new(RefCell::new(tracer));
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default(), Some(tracer.clone()))
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    instance
        .invoke_export_trace(
            f_name,
            &args
                .into_iter()
                .map(|v| RuntimeValue::I32(v))
                .collect::<Vec<_>>(),
            &mut NopExternals,
            tracer.clone(),
        )
        .expect("failed to execute export");

    let tracer_str = format!("{:?}", (*tracer).borrow());
    tracer_str
}
