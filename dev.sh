#!/bin/bash

cargo build && ./target/debug/memos "$@"
