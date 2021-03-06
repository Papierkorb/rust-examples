rust-examples [![Ohloh statistics](http://www.ohloh.net/p/rust-examples/widgets/project_thin_badge.gif)](https://www.ohloh.net/p/rust-examples)
=============

[![Build Status](http://travis-ci.org/eliovir/rust-examples.png?branch=master)](https://travis-ci.org/eliovir/rust-examples)

[Rust-examples](https://github.com/eliovir/rust-examples) is a repository to
gather example codes from tutorial and other documentations of
[Rust](http://www.rust-lang.org/) into files, ready to compile.

Examples are tested with version 0.10-pre.


## Files

* [Homepage](http://www.rust-lang.org/)
    * `what_it_looks_like.rs`
* [Tutorial]
    * 2.1 Compiling your first program: `tutorial-02_1-hello.rs`
    * 3 Syntax basics: `tutorial-03-syntax_basics.rs`
    * 4.2 Pattern matching: `tutorial-04_2-pattern-matching.rs`
    * 4.3 Loops (`for`, `while`, `loop`): `tutorial-04_3-loops.rs`
    * 5.1 Structs: `tutorial-05_1-structs.rs`
    * 5.2 Enums: `tutorial-05_2-enum.rs`
    * 5.3 Tuples: `tutorial-05_3-tuples.rs`
    * 15 Closures: `tutorial-15-closure.rs`
    * 15.3 Do syntax: `tutorial-15_3-do.rs`
    * 16 Methods, with *constructor*: `tutorial-16-methods.rs`
    * 17 Generics: `tutorial-17-generics.rs`
* [Rust Tasks and Communication]
    * 2 Basics: `tutorial-tasks-02-basics.rs`
    * 2.1 Communication: `tutorial-tasks-02_1-communication.rs`
    * 2.2 Backgrounding computations: Futures: `tutorial-tasks-02_2-backgrounding_computations.rs`
* [Doc unit testing]
    * Unit testing in Rust: `unittests.rs`
* API
    * Program to an 'interface', not an 'implementation', by [Josh Davis](http://joshldavis.com/2013/07/01/program-to-an-interface-fool/): `lang-interface.rs`
    * Lambda expressions: `lang-lambda.rs`
    * Pointer snippets from Dave Herman's talk: `lang-pointers.rs`
    * [getopts](http://static.rust-lang.org/doc/master/getopts/index.html): `api-getopts.rs`
    * [std::from_str::FromStr](http://static.rust-lang.org/doc/master/std/from_str/trait.FromStr.html): `api-std-from_str.rs`
    * [std::hashmap::HashMap](http://static.rust-lang.org/doc/master/std/hashmap/struct.HashMap.html): `api-std-hashmap.rs`
    * [std::io::File](http://static.rust-lang.org/doc/master/std/io/index.html): `api-std-io-file.rs`
    * [std::vec](http://static.rust-lang.org/doc/master/std/vec/index.html): OwnedVector, 2D-arrays, ...: `api-std-vec.rs`
* Some new files:
    * `Makefile` to compile, run tests and run benchmarks
    * `.travis.yml` to add the repository to [Travis CI](https://travis-ci.org/eliovir/rust-examples) and [Rust CI](http://www.rust-ci.org/p/90/)
    * A library and its unit tests and benchmarks for 2 Fibonacci functions (a reccursive and a non reccursive): `fibonacci.rs`
    * A struct to manage dates: `date.rs`
    * A struct to manage INI files: `inifile.rs`
    * Design pattern Decorator: `design_pattern-decorator.rs`

[Tutorial]: http://static.rust-lang.org/doc/master/tutorial.html
[The Rust Reference Manual]: http://static.rust-lang.org/doc/master/rust.html
[Rust Tasks and Communication]: http://static.rust-lang.org/doc/master/guide-tasks.html
[Doc unit testing]: http://static.rust-lang.org/doc/master/guide-testing.html


# Compile and running it

You will need the version 0.10-pre of the rust compiler.
If you encounter problems, make sure you have the right version before creating an issue.

The simplest way to build **rust-examples** is to do a clone and use ``make`` to compile:


    git clone https://github.com/eliovir/rust-examples
    cd rust-examples
    make

To run tests and benchmarks:

    make tests
    make bench

To get help on commands:

    make help

## Contributing

1. Fork it (`git clone https://github.com/eliovir/rust-examples`)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Make your changes, and add tests for them
4. Test your changes (`make test`)
5. Commit your changes (`git commit -am 'Add some feature'`)
6. Push to the branch (`git push origin my-new-feature`)
7. Create new Pull Request

## License

Rust is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

These codes are distributed under the MIT license.

See LICENSE for details.
