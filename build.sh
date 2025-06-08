#!/bin/bash
set -e

cargo build --release

mv target/release/freem.exe .

upx --ultra-brute --lzma freem.exe
