# Lyrem

Lyrem is a Rust-based security analysis tool for Linux ELF binaries.

It inspects executable files and shared libraries to report relevant metadata,
build information, and linking properties.

Lyrem does not try to prove that a binary is vulnerable. Its goal is to provide
a clear and structured overview of how a Linux binary was built.

## Objective V1

### File Information

- Name
- Path
- Size

### ELF Information

- ELF class: ELF32 / ELF64
- Architecture
- Binary type
- Entry point

### Build Information

- Compiler, if available
- Build ID, if available

### Linking

- Static / Dynamic

## FuturWork

- Add colors and help
- refactor and optimizing


## License

Licensed under the Apache License, Version 2.0.
See the LICENSE file for details.
