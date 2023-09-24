# pypx-DICOMweb

A server implementing DICOMweb for data received by [pypx-listener](https://github.com/FNNDSC/pypx-listener).

WIP

## Code Outline

- `main.rs` is the driver which load the configuration and runs the server.
- `router.rs` interfaces between `axum` and `pypx_reader.rs`
- `pypx_reader.rs` provides an API for a `pypx`-organized directory of JSON and DICOM files.
- `json_files.rs` and `translate.rs` define helper functions for `pypx_reader.rs`
