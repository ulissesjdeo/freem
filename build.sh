#!/bin/bash
set -e

cargo build --release

upx --ultra-brute --lzma target/release/freem.exe
