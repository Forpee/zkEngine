extern crate wasmi;

use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasmi::{
    nan_preserving_float::{F32, F64},
    ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue,
};

#[wasm_bindgen]
pub fn read_files(
    wasm_mod: &[u8],
    f_name: &str,
    fn_inputs_arr: JsValue,
    datatype: Vec<String>,
) -> String {
    let fn_inputs_arr: Vec<f64> = serde_wasm_bindgen::from_value(fn_inputs_arr).unwrap();

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

    let fn_inputs_arr = &fn_inputs_arr
        .into_iter()
        .zip(datatype)
        .map(|(v, type_str)| {
            // Convert the value to the specified type
            match type_str.as_str() {
                "i32" => RuntimeValue::I32(v as i32),
                "f32" => RuntimeValue::F32(F32::from_float(v as f32)),
                "f64" => RuntimeValue::F64(F64::from_float(v)),
                "i64" => RuntimeValue::I64(v as i64),
                _ => panic!("Unsupported data type: {}", type_str),
            }
        })
        .collect::<Vec<_>>();

    instance
        .invoke_export_trace(f_name, fn_inputs_arr, &mut NopExternals, tracer.clone())
        .expect("failed to execute export");

    let tracer_str = format!("{:?}", (*tracer).borrow());
    tracer_str
}
