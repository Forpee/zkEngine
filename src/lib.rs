extern crate wat;
extern crate wasmi;

use wasm_bindgen::prelude::*;
use core::cell::RefCell;
use std::rc::Rc;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
// Our Add function
// wasm-pack requires "exported" functions
// to include #[wasm_bindgen]
#[wasm_bindgen]
pub fn read_files() {
  // Parse WAT (WebAssembly Text format) into wasm bytecode.
  let wat_str =  r#"
            (module
                (memory $0 1)
                (export "memory" (memory $0))
                (export "fibonacci" (func $fibonacci))
                (func $fibonacci (; 0 ;) (param $0 i32) (result i32)
                 (block $label$0
                  (br_if $label$0
                   (i32.ne
                    (i32.or
                     (local.get $0)
                     (i32.const 1)
                    )
                    (i32.const 1)
                   )
                  )
                  (return
                   (local.get $0)
                  )
                 )
                 (i32.add
                  (call $fibonacci
                   (i32.add
                    (local.get $0)
                    (i32.const -1)
                   )
                  )
                  (call $fibonacci
                   (i32.add
                    (local.get $0)
                    (i32.const -2)
                   )
                  )
                 )
                )
               )
            "#;
    let wasm_binary = wat::parse_str(wat_str).unwrap();

    let mut tracer = wasmi::tracer::Tracer::default();

    // Load wasm binary and prepare it for instantiation.
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    // Instantiate a module with empty imports and
    // asserting that there is no `start` function.
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    tracer.register_module_instance(&instance);
    let tracer = Rc::new(RefCell::new(tracer));

    // Finally, invoke exported function "test" with no parameters
    // and empty external function executor.
    assert_eq!(
        instance
            .invoke_export_trace(
                "fibonacci",
                &[wasmi::RuntimeValue::I32(6)],
                &mut NopExternals,
                tracer.clone(),
            )
            .expect("failed to execute export"),
        Some(RuntimeValue::I32(8)),
    );

    let tracer_str = format!("{:?}", (*tracer).borrow());
    log(&tracer_str);
}