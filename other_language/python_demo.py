#!/usr/bin/env python3

import sys, ctypes
from ctypes import c_int32

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
rust_lib = ctypes.cdll.LoadLibrary(prefix + "rust" + extension)

rust_lib.add.argtypes = (c_int32, c_int32)
rust_lib.add.restype = c_int32

print("add: %d" % rust_lib.add(1, 2))