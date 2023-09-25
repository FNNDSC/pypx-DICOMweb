# pypx-DICOMweb

A server implementing DICOMweb\* for query and retrieval of DICOM data
from a directory managed by [pypx-listener](https://github.com/FNNDSC/pypx-listener).

\*Specifically, this project targets the subset of DICOMweb necessary to get
things working with [OHIF](https://github.com/OHIF/Viewers).

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
- `pypx_reader.rs` provides an API for a `pypx`-organized directory of JSON and DICOM files
- `json_files.rs` and `translate.rs` define helper functions for `pypx_reader.rs`
- `dicom.rs` defines helper functions for reading DICOM files

## OHIF Configuration

```javascript
window.config = {
  // -- snip --
  dataSources: [
    {
      namespace: '@ohif/extension-default.dataSourcesModule.dicomweb',
      sourceName: 'pypx',
      configuration: {
        friendlyName: 'Existing pypx-organized DICOM files',
        name: 'pypx',
        wadoUriRoot: 'http://localhost:4006/dicomweb',
        qidoRoot: 'http://localhost:4006/dicomweb',
        wadoRoot: 'http://localhost:4006/dicomweb',
        qidoSupportsIncludeField: false,
        supportsReject: false,
        imageRendering: 'wadors',
        thumbnailRendering: 'wadors',
        enableStudyLazyLoad: true,
        supportsFuzzyMatching: false,
        supportsWildcard: false,
        staticWado: false,
        singlepart: 'bulkdata,video',
        bulkDataURI: {
          enabled: true,
          relativeResolution: 'studies',
        },
        omitQuotationForMultipartRequest: true
      },
    },
  ]
}
```

## Performance Considerations

`pypx` itself is filesystem-based, hence queries (for studies and series) involve directory traversal.
Moreover, it is necessary to read and parse JSON files to extract information.
These operations are considered slow and can benefit from a caching proxy server (TBA).

## TODO

- etag
- add PatientName to study query results
