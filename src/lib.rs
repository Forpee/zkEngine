extern crate wasmi;

use core::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

#[wasm_bindgen]
pub fn read_files(wasm_mod: &[u8], f_name: &str, args: Vec<i32>) -> String {
    let mut tracer = wasmi::tracer::Tracer::default();

    // Load wasm binary and prepare it for instantiation.
    let module = wasmi::Module::from_buffer(&wasm_mod).unwrap();

    // Instantiate a module with empty imports and
    // asserting that there is no `start` function.
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .run_start(&mut NopExternals)
        .unwrap();

    tracer.register_module_instance(&instance);
    let tracer = Rc::new(RefCell::new(tracer));

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
