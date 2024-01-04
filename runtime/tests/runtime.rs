/// Points to the `template-barebones` program binary.
const BAREBONES_COMPONENT_WASM: &[u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/template_barebones.wasm");

/// Points to the `infinite-loop` program binary.
const INFINITE_LOOP_WASM: &[u8] =
    include_bytes!("../../target/wasm32-unknown-unknown/release/infinite_loop.wasm");

use ec_runtime::{Runtime, SignatureRequest};

#[test]
fn test_barebones_component() {
    let mut runtime = Runtime::default();

    // The barebones example simply validates that the length of the data to be signed is greater than 10.
    let longer_than_10 = "asdfasdfasdfasdf".to_string();
    let signature_request = SignatureRequest {
        message: longer_than_10.into_bytes(),
        auxilary_data: None,
    };

    let res = runtime.evaluate(BAREBONES_COMPONENT_WASM, &signature_request);
    assert!(res.is_ok());
}

#[test]
fn test_barebones_component_fails_with_data_length_less_than_10() {
    let mut runtime = Runtime::default();

    // Since the barebones example verifies that the length of the data to be signed is greater than 10, this should fail.
    let shorter_than_10 = "asdf".to_string();
    let signature_request = SignatureRequest {
        message: shorter_than_10.into_bytes(),
        auxilary_data: None,
    };

    let res = runtime.evaluate(BAREBONES_COMPONENT_WASM, &signature_request);
    assert!(res.is_err());
}

#[test]
fn test_empty_bytecode_fails() {
    let mut runtime = Runtime::default();

    let signature_request = SignatureRequest {
        message: vec![],
        auxilary_data: None,
    };

    let res = runtime.evaluate(&[], &signature_request);
    assert_eq!(res.unwrap_err().to_string(), "Bytecode length is zero");
}

#[test]
fn test_infinite_loop() {
    let mut runtime = Runtime::default();

    let signature_request = SignatureRequest {
        message: vec![],
        auxilary_data: None,
    };

    let res = runtime.evaluate(INFINITE_LOOP_WASM, &signature_request);
    assert_eq!(res.unwrap_err().to_string(), "Runtime error: error while executing at wasm backtrace:\n    0:  0x18a - <unknown>!evaluate");
}
