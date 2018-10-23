// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/set_local.wast
#![allow(
    warnings,
    dead_code
)]
use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, VmCtx, Export};
use super::_common::spectest_importobject;
use wabt::wat2wasm;


// Line 3
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (type (;1;) (func (param i32)))
      (type (;2;) (func (param i64)))
      (type (;3;) (func (param f32)))
      (type (;4;) (func (param f64)))
      (type (;5;) (func (param i64 f32 f64 i32 i32)))
      (type (;6;) (func (param i64 f32 f64 i32 i32) (result i64)))
      (func (;0;) (type 0)
        (local i32)
        i32.const 0
        set_local 0)
      (func (;1;) (type 0)
        (local i64)
        i64.const 0
        set_local 0)
      (func (;2;) (type 0)
        (local f32)
        f32.const 0x0p+0 (;=0;)
        set_local 0)
      (func (;3;) (type 0)
        (local f64)
        f64.const 0x0p+0 (;=0;)
        set_local 0)
      (func (;4;) (type 1) (param i32)
        i32.const 10
        set_local 0)
      (func (;5;) (type 2) (param i64)
        i64.const 11
        set_local 0)
      (func (;6;) (type 3) (param f32)
        f32.const 0x1.633334p+3 (;=11.1;)
        set_local 0)
      (func (;7;) (type 4) (param f64)
        f64.const 0x1.8666666666666p+3 (;=12.2;)
        set_local 0)
      (func (;8;) (type 5) (param i64 f32 f64 i32 i32)
        (local f32 i64 i64 f64)
        i64.const 0
        set_local 0
        f32.const 0x0p+0 (;=0;)
        set_local 1
        f64.const 0x0p+0 (;=0;)
        set_local 2
        i32.const 0
        set_local 3
        i32.const 0
        set_local 4
        f32.const 0x0p+0 (;=0;)
        set_local 5
        i64.const 0
        set_local 6
        i64.const 0
        set_local 7
        f64.const 0x0p+0 (;=0;)
        set_local 8)
      (func (;9;) (type 6) (param i64 f32 f64 i32 i32) (result i64)
        (local f32 i64 i64 f64)
        f32.const -0x1.333334p-2 (;=-0.3;)
        set_local 1
        i32.const 40
        set_local 3
        i32.const -7
        set_local 4
        f32.const 0x1.6p+2 (;=5.5;)
        set_local 5
        i64.const 6
        set_local 6
        f64.const 0x1p+3 (;=8;)
        set_local 8
        get_local 0
        f64.convert_u/i64
        get_local 1
        f64.promote/f32
        get_local 2
        get_local 3
        f64.convert_u/i32
        get_local 4
        f64.convert_s/i32
        get_local 5
        f64.promote/f32
        get_local 6
        f64.convert_u/i64
        get_local 7
        f64.convert_u/i64
        get_local 8
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        i64.trunc_s/f64)
      (export \"type-local-i32\" (func 0))
      (export \"type-local-i64\" (func 1))
      (export \"type-local-f32\" (func 2))
      (export \"type-local-f64\" (func 3))
      (export \"type-param-i32\" (func 4))
      (export \"type-param-i64\" (func 5))
      (export \"type-param-f32\" (func 6))
      (export \"type-param-f64\" (func 7))
      (export \"type-mixed\" (func 8))
      (export \"write\" (func 9)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

// Line 68
fn l68_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-local-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, ());
}

// Line 69
fn l69_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-local-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, ());
}

// Line 70
fn l70_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-local-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, ());
}

// Line 71
fn l71_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-local-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(&vm_context);
    assert_eq!(result, ());
}

// Line 73
fn l73_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-param-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(2 as i32, &vm_context);
    assert_eq!(result, ());
}

// Line 74
fn l74_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-param-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(3 as i64, &vm_context);
    assert_eq!(result, ());
}

// Line 75
fn l75_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-param-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, &VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(4.4 as f32, &vm_context);
    assert_eq!(result, ());
}

// Line 76
fn l76_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-param-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, &VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(5.5 as f64, &vm_context);
    assert_eq!(result, ());
}

// Line 79
fn l79_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("type-mixed") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, f32, f64, i32, i32, &VmCtx) = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(1 as i64, 2.2 as f32, 3.3 as f64, 4 as i32, 5 as i32, &vm_context);
    assert_eq!(result, ());
}

// Line 85
fn l85_assert_return_invoke(result_object: &ResultObject) {
    let func_index = match result_object.module.info.exports.get("write") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, f32, f64, i32, i32, &VmCtx) -> i64 = get_instance_function!(result_object.instance, func_index);
    let vm_context = result_object.instance.generate_context();
    let result = invoke_fn(1 as i64, 2.0 as f32, 3.3 as f64, 4 as i32, 5 as i32, &vm_context);
    assert_eq!(result, 56 as i64);
}

// Line 95
#[test]
fn l95_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 126, 3, 2, 1, 0, 10, 10, 1, 8, 1, 1, 127, 65, 0, 33, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 101
#[test]
fn l101_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 14, 1, 12, 1, 1, 125, 67, 0, 0, 0, 0, 33, 0, 69, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 107
#[test]
fn l107_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 2, 1, 124, 1, 126, 66, 0, 33, 1, 154, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 114
#[test]
fn l114_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 9, 1, 7, 1, 1, 127, 1, 33, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 118
#[test]
fn l118_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 1, 1, 127, 67, 0, 0, 0, 0, 33, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 122
#[test]
fn l122_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 17, 1, 15, 1, 1, 125, 68, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 126
#[test]
fn l126_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 19, 1, 17, 2, 1, 124, 1, 126, 68, 0, 0, 0, 0, 0, 0, 0, 0, 33, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 134
#[test]
fn l134_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 1, 127, 1, 126, 3, 2, 1, 0, 10, 6, 1, 4, 0, 32, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 138
#[test]
fn l138_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 125, 0, 3, 2, 1, 0, 10, 7, 1, 5, 0, 32, 0, 69, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 142
#[test]
fn l142_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 124, 126, 0, 3, 2, 1, 0, 10, 7, 1, 5, 0, 32, 1, 154, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 147
#[test]
fn l147_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 10, 7, 1, 5, 0, 1, 33, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 151
#[test]
fn l151_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 10, 11, 1, 9, 0, 67, 0, 0, 0, 0, 33, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 155
#[test]
fn l155_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 125, 0, 3, 2, 1, 0, 10, 15, 1, 13, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 159
#[test]
fn l159_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 124, 126, 0, 3, 2, 1, 0, 10, 15, 1, 13, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 33, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 167
#[test]
fn l167_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 10, 1, 8, 2, 1, 127, 1, 126, 32, 3, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 171
#[test]
fn l171_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 2, 1, 127, 1, 126, 32, 247, 164, 234, 6, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 176
#[test]
fn l176_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 127, 126, 0, 3, 2, 1, 0, 10, 6, 1, 4, 0, 32, 2, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 180
#[test]
fn l180_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 14, 1, 12, 2, 1, 127, 1, 126, 32, 247, 242, 206, 212, 2, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 185
#[test]
fn l185_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 10, 10, 1, 8, 2, 1, 127, 1, 126, 32, 3, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 189
#[test]
fn l189_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 126, 0, 3, 2, 1, 0, 10, 13, 1, 11, 2, 1, 127, 1, 126, 32, 247, 168, 153, 102, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 194
#[test]
fn l194_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 125, 0, 3, 2, 1, 0, 10, 13, 1, 11, 1, 1, 127, 67, 0, 0, 0, 0, 33, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 198
#[test]
fn l198_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 126, 127, 0, 3, 2, 1, 0, 10, 13, 1, 11, 1, 1, 125, 67, 0, 0, 0, 0, 33, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 202
#[test]
fn l202_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 126, 0, 3, 2, 1, 0, 10, 12, 1, 10, 2, 1, 124, 1, 126, 66, 0, 33, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    // We group the calls together
    l68_assert_return_invoke(&result_object);
    l69_assert_return_invoke(&result_object);
    l70_assert_return_invoke(&result_object);
    l71_assert_return_invoke(&result_object);
    l73_assert_return_invoke(&result_object);
    l74_assert_return_invoke(&result_object);
    l75_assert_return_invoke(&result_object);
    l76_assert_return_invoke(&result_object);
    l79_assert_return_invoke(&result_object);
    l85_assert_return_invoke(&result_object);
}
