# Lyrem

Lyrem is a Rust-based security analysis tool for Linux ELF binaries.

It parses ELF metadata and reports security-relevant properties such as NX, PIE,
RELRO, dynamic linking, interpreter presence, and RWX segments.

Lyrem does **not** try to prove that a binary is vulnerable. Its goal is to
provide a clear and structured overview of how a Linux binary was built.

## Status

![CI](https://github.com/VBrochard/Lyrem/actions/workflows/ci.yml/badge.svg)
![Docs](https://github.com/VBrochard/Lyrem/actions/workflows/docs.yml/badge.svg)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)
![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)

Documentation: <https://vbrochard.github.io/Lyrem/>

## Features

| Area | Supported |
|---|---|
| ELF Header parsing | ELF class, endianness, ABI, architecture, binary type, entry point |
| Program Headers | Type, permissions, offsets, virtual/physical addresses, sizes, alignment |
| Dynamic Section | Dynamic entries used for linking and RELRO analysis |
| Security Analysis | NX, PIE, RELRO, RWX segments, dynamic segment, interpreter |
| Output modes | Human-readable output, full report, JSON |
| Testing | Integration tests with generated ELF assets |
| CI | Format check, Clippy, tests, coverage, documentation |

## Security Checks

| Check | Description |
|---|---|
| NX | Detects whether the stack is marked executable through `GNU_STACK` |
| PIE | Detects position-independent executables using ELF type and interpreter data |
| RELRO | Detects no, partial, or full RELRO using Program Headers and Dynamic Section entries |
| RWX segments | Reports `LOAD` segments that are readable, writable, and executable |
| Dynamic segment | Detects whether the binary contains a `PT_DYNAMIC` Program Header |
| Interpreter | Detects whether the binary contains a `PT_INTERP` Program Header |

## Usage

Analyze a binary:

```bash
lyrem /usr/bin/bash
```

By default, Lyrem displays a security-focused report:

```text
Security Analysis
-----------------
NX               : Enabled
PIE              : Enabled
Dynamic Segment  : Yes
RELRO            : Full
Interpreter      : Yes
RWX Segments     : None
```

Display the full ELF report:

```bash
lyrem --full /usr/bin/bash
```

Generate JSON output:

```bash
lyrem --json /usr/bin/bash
```

Redirect JSON output to a file:

```bash
lyrem --json /usr/bin/bash > bash.json
```

Show available options:

```bash
lyrem --help
```

## Example

```bash
lyrem --full /usr/bin/bash
```

The full report includes:

- file information;
- ELF header metadata;
- security analysis;
- Program Headers.

## Installation

Build from source:

```bash
git clone https://github.com/VBrochard/Lyrem.git
cd Lyrem
cargo build --release
```

The release binary will be available at:

```bash
target/release/lyrem
```

You can run it directly:

```bash
./target/release/lyrem /usr/bin/bash
```

## Development

Lyrem uses generated ELF test assets. Before running the test suite, build them:

```bash
make -C tests/assets/src
```

Then run tests :

```bash
cargo test
```

## Test Assets

The test suite uses small generated ELF binaries to validate parsing and security
analysis behavior.

| Asset | Purpose |
|---|---|
| `hello` | Non-PIE executable |
| `hello.o` | Relocatable object file |
| `libhello.so` | Shared library |
| `hello_pie_partial_relro` | PIE binary with partial RELRO |
| `hello_nopie_full_relro` | Non-PIE binary with full RELRO |
| `hello_execstack` | Binary with executable stack |
| `hello_norelro` | Binary without RELRO |
| `rwx` | Assembly-generated binary with a RWX `LOAD` segment |
| `not_elf.txt` | Invalid non-ELF input |

Generated binaries are not committed to the repository. They are produced from
the sources in `tests/assets/src`.

## Project Goals

Lyrem 1.0.0 focuses on:

- understanding the ELF format through manual parsing;
- building a security-oriented binary analysis tool;
- keeping the parser independent from external ELF parsing libraries;
- providing a clean CLI and JSON output;
- maintaining tests, CI, coverage, and generated documentation.

## Limitations

Lyrem currently focuses on Linux ELF binaries, mainly tested on x86-64.

It does not perform full vulnerability detection, exploitability analysis, or
deep symbolic inspection. Its purpose is to extract and interpret ELF metadata
relevant to binary hardening.

## Future Work

- Section Header parsing
- Shared library name extraction
- Symbol table parsing
- Build ID and ELF notes
- More output modes
- Optional colored CLI output
- Broader architecture and ELF32 test coverage
- Distribution packaging

## License

Licensed under the Apache License, Version 2.0.

See the [LICENSE](LICENSE) file for details.
