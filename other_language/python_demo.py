#!/usr/bin/env python3

import sys, ctypes
from ctypes import c_int32, c_uint32, c_char_p

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
rust_lib = ctypes.cdll.LoadLibrary(prefix + "rust" + extension)

rust_lib.add.argtypes = [c_int32, c_int32]
rust_lib.add.restype = c_int32

rust_lib.count_chars.argtypes = [c_char_p]
rust_lib.count_chars.restype = c_uint32

print("add: %d" % rust_lib.add(1, 2))
print("\\ncount_chars: %d" % rust_lib.count_chars("Hello, World!".encode('utf-8')))