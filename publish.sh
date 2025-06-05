#!/bin/bash
set -e

cargo package

cargo publish
