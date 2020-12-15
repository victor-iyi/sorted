#!/bin/bash
set -ex

# Install Rust.
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"

# Build for these wheels Python Versions.
for PYBIN in /opt/python/{cp35-cp35m,cp36-cp36m,cp37-cp37m,cp38-cp38,cp39-cp39}/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"

    "${PYBIN}/pip" install -U setuptools-rust
    "${PYBIN}/python" setup.py bdist_wheel
    rm -rf build/*
done

for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
done

# Keep only manylinux wheels
rm dist/*-linux_*
