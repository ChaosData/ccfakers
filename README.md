# ccfakers
Example using rust to spoof c++ class implementations and link c++ against rust shared objects.

# Build/Usage Example
```
$ cd rs/ccfakers
$ rustc --version
rustc 1.0.0 (a59de37e9 2015-05-13) (built 2015-05-14)
$ cargo build --release
   Compiling ccfakers v0.1.0 (file:///home/jtd/github/ccfakers/rs/ccfakers)
$ cd ../../cc
$ clang++ -std=c++14 -stdlib=libc++ -Wall -Wextra -L ../rs/ccfakers/target/release -o main main.cc -lccfakers
$ LD_LIBRARY_PATH=../rs/ccfakers/target/release ./main 44
49
47
```
