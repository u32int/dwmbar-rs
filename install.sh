#!/bin/bash

cargo build --release
sudo cp ./target/release/dwmbar /usr/local/bin/
