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

### Security Analysis

- NX status
- PIE status
- RELRO presence
- Dynamic segment detection
- Interpreter detection
- RWX LOAD segment detection

## Usage

Lyrem takes an ELF binary path as input:

```bash
lyrem /usr/bin/bash
```

By default, it displays a security-focused analysis of the binary.
To display the full human-readable report, including file information, ELF
metadata, security analysis, and Program Headers:

```bash
lyrem --full /usr/bin/bash
```

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

- Improve security analysis accuracy
- Distinguish partial and full RELRO using the Dynamic Section
- Section Headers
- Shared libraries
- Dynamic Section
- Symbol tables
- CLI improvements (colors, output modes)
- Additional tests and coverage
- Replace committed test binaries with generated test assets
- Add scripts to build ELF test fixtures from source
- Refactoring and optimization

## License

Licensed under the Apache License, Version 2.0.
See the LICENSE file for details.
