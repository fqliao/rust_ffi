#!/usr/bin/env python3

import sys, ctypes
from ctypes import c_int32, c_uint32, c_char_p, c_void_p, Structure, POINTER

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
rust_lib = ctypes.cdll.LoadLibrary(prefix + "rust" + extension)

rust_lib.add.argtypes = [c_int32, c_int32]
rust_lib.add.restype = c_int32

rust_lib.hello.argtypes = [c_char_p]
rust_lib.hello.restype = c_void_p
rust_lib.hello_free.argtypes = [c_void_p]

class Person(Structure):
    pass

rust_lib.get_person.argtypes = [c_char_p, c_uint32]
rust_lib.get_person.restype = POINTER(Person)

rust_lib.person_get_name.argtypes = [POINTER(Person)]
rust_lib.person_get_name.restype = c_void_p

rust_lib.person_get_age.argtypes = [POINTER(Person)]
rust_lib.person_get_age.restype = c_uint32

rust_lib.person_free.argtypes = [POINTER(Person)]

print("add: %d" % rust_lib.add(1, 2))

def python_hello(str):
    str_ptr = rust_lib.hello(str.encode('utf-8'))
    try:
        return ctypes.cast(str_ptr, ctypes.c_char_p).value.decode('utf-8')
    finally:
        rust_lib.hello_free(str_ptr)
print("\\nhello: %s" % python_hello("World"))

person = rust_lib.get_person("Alice", 12)
try:
    str_ptr = rust_lib.person_get_name(person)
    name = ctypes.cast(str_ptr, ctypes.c_char_p).value.decode('utf-8')
    age = rust_lib.person_get_age(person)
    print("\\nget_person: {name=\"%s\", age=%d} " % (name, age))
finally:
    rust_lib.person_free(person)
