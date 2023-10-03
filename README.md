# pypx-rs

[![CI](https://github.com/FNNDSC/pypx-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/FNNDSC/pypx-rs/actions/workflows/ci.yml)
[![MIT License](https://img.shields.io/github/license/fnndsc/pypx-rs)](./LICENSE)

Monorepo of [pypx](https://github.com/fnndsc/pypx)-related components (re-)written in Rust.

`pypx` is a suite of Python scripts used by our lab, the
[FNNDSC](https://fnndsc.org), for interacting with the hospital's PACS server.
This repo, `pypx-rs`, contains:

- [pypx](./pypx) (the crate): Rust type definitions for `pypx` schemas
- [pypx-DICOMweb](./pypx-DICOMweb): a server implementing the DICOMweb API for a `pypx`-organized directory

## TODO

Move https://github.com/FNNDSC/pypx-listener in here
