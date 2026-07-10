# Lyrem

Lyrem is a Rust-based security analysis tool for Linux ELF binaries.

It inspects executable files and shared libraries to report relevant metadata,
build information, and linking properties.

Lyrem does not try to prove that a binary is vulnerable. Its goal is to provide
a clear and structured overview of how a Linux binary was built.

## Objective 1.0.0

### File Information

- Name
- Path
- Size

### ELF Information

- ELF class (ELF32 / ELF64)
- Endianness
- ABI
- Architecture
- Binary type
- Entry point

### Program Headers

- Number of Program Headers
- Program type
- Memory permissions (R / W / E)
- File offset
- Virtual address
- Physical address
- File size
- Memory size
- Alignment

## Usage

Lyrem takes an ELF binary path as input:

```bash
lyrem /usr/bin/bash
```

By default, it displays a human-readable report containing file information,
ELF metadata, and Program Headers.
To obtain machine-readable JSON output:

```bash
lyrem --json /usr/bin/bash
```

The output can also be redirected to a file:

```bash
lyrem --json /usr/bin/bash > bash.json
```

For more information about available options:

```bash
lyrem --help
```

## Future Work

- Security analysis (NX, PIE, RELRO, RWX, etc.)
- Section Headers
- Shared libraries
- Dynamic section
- Symbol tables
- CLI improvements (`--full`, `--help`, colors)
- Additional tests and coverage
- Refactoring and optimization

## License

Licensed under the Apache License, Version 2.0.
See the LICENSE file for details.
