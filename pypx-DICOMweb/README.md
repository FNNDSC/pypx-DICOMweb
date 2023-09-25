# pypx-DICOMweb

A server implementing DICOMweb for data received by [pypx-listener](https://github.com/FNNDSC/pypx-listener).


## Development

First, download some example data:

```shell
../example_data/download.sh
```

Then run the server:

```shell
export PYPX_LOG_DIR=../example_data/samples/pypx/log
export PYPX_DATA_DIR=../example_data/samples/pypx/data
export PYPX_REPACK_DATA_MOUNTPOINT=/tmp/dicom/data
export RUST_LOG=pypx_dicomweb=DEBUG
env PORT=4006 cargo run
```

## Code Outline

- `main.rs` is the driver which load the configuration and runs the server.
- `router.rs` interfaces between `axum` and `pypx_reader.rs`
- `pypx_reader.rs` provides an API for a `pypx`-organized directory of JSON and DICOM files.
- `json_files.rs` and `translate.rs` define helper functions for `pypx_reader.rs`

## TODO

- etag
- add PatientName to study query results
