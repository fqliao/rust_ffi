#!/usr/bin/env python3

import sys, ctypes
from ctypes import c_int32, c_uint32, c_char_p, c_void_p

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
rust_lib = ctypes.cdll.LoadLibrary(prefix + "rust" + extension)

rust_lib.add.argtypes = [c_int32, c_int32]
rust_lib.add.restype = c_int32

rust_lib.hello.argtypes = [c_char_p]
rust_lib.hello.restype = c_void_p
rust_lib.hello_free.argtypes = [c_void_p]

print("add: %d" % rust_lib.add(1, 2))

def python_hello(str):
    str_ptr = rust_lib.hello(str.encode('utf-8'))
    try:
        return ctypes.cast(str_ptr, ctypes.c_char_p).value.decode('utf-8')
    finally:
        rust_lib.hello_free(str_ptr)
print("\\nhello: %s" % python_hello("World"))