# CSharp (C#) Protobuf Compiler
This program is a wrapper around [protoc](https://github.com/protocolbuffers/protobuf).
It's goal is to compile proto files to C# source files, as done by `protoc`, but to keep the 
original directory structure intect. E.g. `$SRC/protos/foo/bar/baz/quix.proto`, will be generated
at `$OUT/protos/foo/bar/baz/Quix.cs`

## Installation
Requirments:
- Protocol buffer compiler (`protoc`)
- Rust toolchain (`cargo` and `rustc`)

Use `cargo install` to install this program.

## Usage
Arguments:
- `--server/-s` The path to the source directory containing all source files.
- `--include/-i` Folder to include (passed as `-I` to protoc; usually the same as `--server`)
- `--out/-o` The base directory to output to

Use `--help` for more info.

## License
This project is licensed under the MIT license.