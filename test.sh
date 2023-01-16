#!/bin/bash

cargo build --release
time (echo eerrr | ./target/release/erettsegi_emelt_2006_10_rust)

