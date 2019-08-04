extern crate libc;
//use libc::uint32_t;

extern crate jni;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jint, jobject};
use jni::JNIEnv;

/// rust for C++, Python and Nodejs
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    rust_add(a, b)
}

/// rust for java
#[no_mangle]
pub extern "system" fn Java_JavaDemo_add(_env: JNIEnv, _class: JClass, a: jint, b: jint) -> jint {
    rust_add(a, b)
}

// rust fn
fn rust_add(a:i32, b:i32)->i32{
    a + b
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(rust_add(1,2), 3);
    }
}