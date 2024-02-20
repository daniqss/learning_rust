from ctypes import cdll
lib = cdll.LoadLibrary('./target/release/libconcurrency_and_ffi.so')

iterations = 1000
counts = 100
threads = 10
multithread = lib.study_times(iterations, counts, threads)

print(f"Rust: {multithread} microseconds with {threads} threads and {counts} counts")


iterations = 1000
counts = counts * threads
threads = 1
monothread = lib.study_times(iterations, counts, threads)

print(f"Rust: {monothread} microseconds with {threads} threads and {counts} counts")