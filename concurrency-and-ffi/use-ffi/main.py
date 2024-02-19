from ctypes import cdll

lib = cdll.LoadLibrary('../target/release/libconcurrency_and_ffi.so')

lib.study_times(1000, 100, 10)
