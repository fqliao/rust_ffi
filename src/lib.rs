extern crate libc;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::str;

// for java
extern crate jni;
use jni::objects::{JClass, JString, JObject, JValue};
use jni::sys::{jint, jstring, jobject};
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
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

pub struct Person {
    name:String,
    age:u32
}

impl Person {
    fn new(name:&str, age:u32) -> Person {
        Person {
            name: name.to_string(),
            age:age
        }
    }
    fn get_name(&self)->String{
        self.name.clone()
    }
    fn get_age(&self)->u32{
        self.age
    }
}

#[no_mangle]
pub extern  "C" fn get_person(name: *const c_char, age:u32) -> *mut Person {
    let c_name = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let r_name = c_name.to_str().unwrap();
    Box::into_raw(Box::new(Person::new(r_name, age)))
}

#[no_mangle]
pub extern  "C" fn person_get_name(ptr:*const Person) -> *mut c_char {
    let person = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let name = person.get_name();
    let c_name = CString::new(name).unwrap();
    c_name.into_raw()
}

#[no_mangle]
pub extern  "C" fn person_get_age(ptr:*const Person) -> u32 {
    let person = unsafe{
        assert!(!ptr.is_null());
        &*ptr
    };
    person.get_age()
}

#[no_mangle]
pub extern  "C" fn person_free(ptr: *mut Person) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

/// rust for java
#[no_mangle]
pub extern "system" fn Java_org_com_fisco_JavaDemo_add(_env: JNIEnv, _class: JClass, a: jint, b: jint) -> jint {
    rust_add(a, b)
}

#[no_mangle]
pub extern "system" fn Java_org_com_fisco_JavaDemo_hello(_env: JNIEnv, _class: JClass, str: JString) -> jstring {
    // transfer jString to rust String
    let r_str: String = _env
        .get_string(str)
        .expect("Couldn't get java string!")
        .into();
    // transfer rust String to jString
    let j_str = _env
        .new_string(rust_hello(&r_str))
        .expect("Couldn't create java string!");
    // transfer jString to jstring
    j_str.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_org_com_fisco_JavaDemo_getPerson(
    _env: JNIEnv,
    _class: JClass,
    name: JString, // java string parameter must be JString
    age: jint,
) -> jobject {
    // 1. call rust fn
    let r_name: String = _env
        .get_string(name)
        .expect("Couldn't get java string!")
        .into();
    let person = Person::new(r_name.as_str(),age as u32);
    let j_name = _env
        .new_string(person.get_name())
        .expect("Couldn't create java string!");
    let j_age:jint = person.get_age() as i32;

    //2. find java class
    let person_class = _env
        .find_class("Lorg/com/fisco/Person;")
        .expect("could not find Person class");

    //3. allocate java object
    let j_person = _env
        .alloc_object(person_class)
        .expect("Could not allocate Person object");

    //4 set field
    _env.set_field(
        j_person,
        "name",
        "Ljava/lang/String;",
        JValue::from(JObject::from(j_name)),
    )
    .expect("Could not set name field");
    _env.set_field(j_person, "age", "I", JValue::from(j_age))
        .expect("Could not set age field");

    //5 return object
    j_person.into_inner()
}

// rust fn
fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

fn rust_hello(str: &str) -> String {
    format!("Hello, {}!", str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(rust_add(1, 2), 3);
    }

    #[test]
    fn test_hello() {
        assert_eq!(rust_hello("World"), "Hello, World!");
    }

    #[test]
    fn test_get_person(){
        let person = Person::new("alice", 12);
        assert_eq!(person.get_name(), "alice");
        assert_eq!(person.get_age(), 12);
    }
}
