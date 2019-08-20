#!/usr/bin/env bash

cd interfacer-http-util && cargo test && cargo publish; cd ..;
cd interfacer-http-attribute && cargo test && cargo publish; cd ..;
cargo test --features "serde-full,encoding,unhtml-html,derive" && cargo publish;
cd interfacer-http-hyper && cargo test && cargo publish; cd ..;