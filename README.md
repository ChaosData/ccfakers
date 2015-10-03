# ccfakers
Example of using rust to spoof c++ class implementations and link c++ directly against rust binaries.

## Changelog

v0.1.0: Initial basic version that uses separate allocator/deallocator functions.

v0.2.0: Example updated to leverage spoofed constructor/destructor and implemented a fake vtable for a base class.

# Build/Usage Example
```
$ cd rs/ccfakers
$ rustc --version
rustc 1.3.0 (9a92aaf19 2015-09-15)
$ cargo build --release
   Compiling libc v0.1.10
   Compiling ccfakers v0.2.0 (file:///home/jtd/rscc/ccfakers/rs/ccfakers)
note: link against the following native artifacts when linking against this static library
note: the order and any duplication can be significant on some platforms, and so may need to be preserved
note: library: c
note: library: m
note: library: dl
note: library: pthread
note: library: rt
note: library: gcc_s
note: library: pthread
note: library: c
note: library: m
$ cd ../../cc
$ clang++ -std=c++14 -stdlib=libc++ -Wall -Wextra -g -o main main.cc -fPIE -pie -fstack-protector-all -Wl,-z,relro,-z,now ../rs/ccfakers/target/release/libccfakers.a -lpthread -ldl
$ valgrind ./main 40
==109497== Memcheck, a memory error detector
==109497== Copyright (C) 2002-2013, and GNU GPL'd, by Julian Seward et al.
==109497== Using Valgrind-3.10.0.SVN and LibVEX; rerun with -h for copyright info
==109497== Command: ./main 40
==109497== 
rust >> IntHolder::IntHolder(long)
name: 9LOLHolder
rust >> IntHolder::add(long)
rust >> IntHolder::value()
45
rust >> IntHolder::sub(long)
rust >> IntHolder::value()
43
rust >> IntHolder::~IntHolder()
==109497== 
==109497== HEAP SUMMARY:
==109497==     in use at exit: 0 bytes in 0 blocks
==109497==   total heap usage: 1 allocs, 1 frees, 16 bytes allocated
==109497== 
==109497== All heap blocks were freed -- no leaks are possible
==109497== 
==109497== For counts of detected and suppressed errors, rerun with: -v
==109497== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 0 from 0)
```
