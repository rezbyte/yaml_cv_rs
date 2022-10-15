# yaml_cv_rs

## Table of Contents
+ [About](#about)
+ [Getting Started](#getting_started)
+ [Usage](#usage)

## About <a name = "about"></a>
This my attempt at porting [kaityo256's yaml_cv](https://github.com/kaityo256/yaml_cv) to Rust.
It supports all the style files the original does.
I wrote this to practice writing in Rust, so I don't intend to maintain this.

It's easier to setup, available as a [download](https://github.com/rezbyte/yaml_cv_rs/releases).
Runs around twice as fast than the original Ruby script.

![screenshot1](sample/academic1.jpg)
![screenshot2](sample/academic2.jpg)

## Getting Started <a name = "getting_started"></a>
These instructions will build a copy of the project up and running on your local machine.

### Prerequisites

You will need [Rust](https://www.rust-lang.org/tools/install) installed.


### Installing

First, build the project in cargo:
```
cargo ship
```

Then copy the: 
- [IPA fonts](https://moji.or.jp/ipafont/ipaex00401/) into `target/release/fonts`.
- [Standard](https://github.com/kaityo256/yaml_cv/blob/main/style.txt) or [academic](https://github.com/kaityo256/yaml_cv/blob/main/academic.txt) style files into `target/release`.
- [Data YAML file](https://github.com/kaityo256/yaml_cv/blob/main/data.yaml) into `target/release`.

After that, `yaml_cv_rs` should be ready for use in `target/release`.

## Usage <a name = "usage"></a>

First [download the IPA mincho & gothic fonts](https://moji.or.jp/ipafont/ipaex00401/).
Extract the IPA fonts into `yaml_cv_rs/fonts`.

You can run the program directly with `./yaml_cv_rs`.
The usage is identical to kaityo256's version.

You can run `yaml_cv_rs --help` to get usage instructions:
```
Usage: yaml_cv_rs [OPTIONS]

Options:
  -i, --input <INPUT>    Path to the input file in YAML format [default: data.yaml]
  -s, --style <STYLE>    Path to the styling file [default: style.txt]
  -o, --output <OUTPUT>  Path to output the final PDF file to [default: output.pdf]
  -h, --help             Print help information
  -V, --version          Print version information
```
