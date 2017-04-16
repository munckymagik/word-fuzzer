# Word Fuzzer - shuffles the middle letters of words

For example:

```
$ word-fuzzer what a wonderful day to be rusting away
waht a wornduefl day to be rntisug aawy
```

## But ... why?

The product of a boring Saturday morning. Gave me an excuse to write some Rust
code and get a feel for string manipulation using slices and so on.

## Set up

* Install the [Rust build environment](https://www.rust-lang.org/en-US/install.html).
* Check out the source using Git.

## Usage

All commands assume you are in a local checkout of the project.

Run the tests:

```
$ cargo test
```

Run in development:

```
$ cargo run then type some words
tehn tpye smoe wdors
```

Install locally:

```
$ cargo install
```

By default Cargo installs the binary to `~/.cargo/bin`:

```
$ which word-fuzzer
/Users/jdoe/.cargo/bin/word-fuzzer
$ word-fuzzer now type some words
now tpye smoe wrods 
```
