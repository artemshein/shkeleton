#!/usr/bin/env bash

cargo test --features cli && \
cargo test --features concurrency && \
cargo test --features failure && \
cargo test --features cli,concurrency,failure
