extern crate libc;
use libc::{c_char};
use std::ffi::CStr;
use std::str;

// for java
extern crate jni;
use jni::objects::{JClass, JString};
use jni::sys::{jint};
use jni::JNIEnv;

/// rust for C++, Python and Nodejs
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    rust_add(a, b)
}

#[no_mangle]
pub extern "C" fn count_chars(str: *const c_char) -> u32 {
    let c_str = unsafe {
        assert!(!str.is_null());

        CStr::from_ptr(str)
    };

    let r_str = c_str.to_str().unwrap();
    rust_count_chars(r_str) as u32
}

/// rust for java
#[no_mangle]
pub extern "system" fn Java_JavaDemo_add(_env: JNIEnv, _class: JClass, a: jint, b: jint) -> jint {
    rust_add(a, b)
}

#[no_mangle]
pub extern "system" fn Java_JavaDemo_countChars(_env: JNIEnv, _class: JClass, str: JString) -> jint {
    let r_str:String = _env.get_string(str).expect("Couldn't get java string!").into();
    rust_count_chars(&r_str) as i32
}

// rust fn
fn rust_add(a:i32, b:i32)->i32{
    a + b
}

fn rust_count_chars(str:&str)->u32{
    str.chars().count() as u32
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(rust_add(1,2), 3);
    }
}