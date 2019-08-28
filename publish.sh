#!/usr/bin/env bash

cargo test --all --features "serde-full,unhtml-html,encoding,derive,runtime,mock"
cd interfacer-http-attribute && cargo publish; cd ..;
cargo publish;
cd interfacer-http-hyper && cargo publish; cd ..;