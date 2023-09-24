# pypx-rs

Monorepo of [pypx](https://github.com/fnndsc/pypx)-related components (re-)written in Rust.

`pypx` is a suite of Python scripts used by our lab, the
[FNNDSC](https://fnndsc.org), for interacting with the hospital's PACS server.
This repo, `pypx-rs`, contains:

- [pypx](./pypx) (the crate): Rust type definitions for `pypx` schemas
- [pypx-DICOMweb](./pypx-DICOMweb): a server implementing the DICOMweb API for a `pypx`-organized directory

## TODO

Move https://github.com/FNNDSC/pypx-listener in here
