# `cxx-test` project

This small project is MRE to reproduce the issue 
[cxx #851](https://github.com/dtolnay/cxx/issues/851)

Reproducing is easy as
```shell
cargo run
```

Results in
```
= note: Undefined symbols for architecture x86_64:
        "rust::cxxbridge1::Vec<LE::Point>::data() const", referenced from:
            rust::cxxbridge1::Vec<LE::Point>::data() in liblibrary-6115a8aed8f71b5c.rlib(lib.o)
        "rust::cxxbridge1::Vec<LE::Point>::size() const", referenced from:
            void rust::cxxbridge1::Vec<LE::Point>::emplace_back<LE::Point const&>(LE::Point const&&&) in liblibrary-6115a8aed8f71b5c.rlib(lib.o)
        "rust::cxxbridge1::Vec<LE::Point>::set_len(unsigned long)", referenced from:
            void rust::cxxbridge1::Vec<LE::Point>::emplace_back<LE::Point const&>(LE::Point const&&&) in liblibrary-6115a8aed8f71b5c.rlib(lib.o)
        "rust::cxxbridge1::Vec<LE::Point>::Vec()", referenced from:
            _shim_getPoints(LE::Game&) in liblibrary-6115a8aed8f71b5c.rlib(lib.o)
        "rust::cxxbridge1::Vec<LE::Point>::reserve_total(unsigned long)", referenced from:
            rust::cxxbridge1::Vec<LE::Point>::reserve(unsigned long) in liblibrary-6115a8aed8f71b5c.rlib(lib.o)
            void rust::cxxbridge1::Vec<LE::Point>::emplace_back<LE::Point const&>(LE::Point const&&&) in liblibrary-6115a8aed8f71b5c.rlib(lib.o)
        "rust::cxxbridge1::Vec<LE::Point>::drop()", referenced from:
            rust::cxxbridge1::Vec<LE::Point>::~Vec() in liblibrary-6115a8aed8f71b5c.rlib(lib.o)
      ld: symbol(s) not found for architecture x86_64
      clang: error: linker command failed with exit code 1 (use -v to see invocation)
```
