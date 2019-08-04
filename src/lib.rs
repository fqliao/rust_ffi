extern crate libc;
use libc::{c_char};
use std::ffi::{CStr, CString};
use std::str;

// for java
extern crate jni;
use jni::objects::{JClass, JString};
use jni::sys::{jint, jstring};
use jni::JNIEnv;

/// rust implementation for C++, Python and Nodejs
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    rust_add(a, b)
}

#[no_mangle]
pub extern "C" fn hello(str: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!str.is_null());
        CStr::from_ptr(str)
    };

    let r_str = c_str.to_str().unwrap();
    let result = rust_hello(r_str);
    let c_result = CString::new(result).unwrap();
    c_result.into_raw()
}

#[no_mangle]
pub extern "C" fn hello_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

/// rust for java
#[no_mangle]
pub extern "system" fn Java_JavaDemo_add(_env: JNIEnv, _class: JClass, a: jint, b: jint) -> jint {
    rust_add(a, b)
}

#[no_mangle]
pub extern "system" fn Java_JavaDemo_hello(_env: JNIEnv, _class: JClass, str: JString) -> jstring {
    // transfer jString to rust String
    let r_str:String = _env.get_string(str).expect("Couldn't get java string!").into();
    // transfer rust String to jString
    let j_str = _env.new_string(rust_hello(&r_str)).expect("Couldn't create java string!");
    // transfer jString to jstring
    j_str.into_inner()
}

// rust fn
fn rust_add(a:i32, b:i32)->i32{
    a + b
}

fn rust_hello(str:&str)->String{
    format!("Hello, {}!", str)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(rust_add(1,2), 3);
    }

    #[test]
    fn test_hello(){ assert_eq!(rust_hello("World"), "Hello, World!");}
}