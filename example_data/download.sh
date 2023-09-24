#!/bin/bash -e
# Purpose: Download example DICOM data from NERC OpenStack Swift

cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null

if [ -e "./samples" ]; then
  echo "samples already exist"
  exit 1
fi

PUBLIC_URL='https://stack.nerc.mghpcc.org:13808/swift/v1/AUTH_2dd3b02b267242d9b28f94a512ea9ede/fnndsc-public/'
exec parallel -j4 --bar "mkdir -p '{//}' && curl -sfo '{}' '$PUBLIC_URL{}'" < examples.txt
