#!/bin/sh

# runs the tests for all 4 combinations of std & unsafe
# stops at the first one that fails

echo ">>> t" && cargo t && \
echo ">>> t no_unsafe" && cargo t --features=no_unsafe && \
echo ">>> t no-default" && cargo t --no-default-features && \
echo ">>> t no-default no_unsafe" && cargo t --no-default-features --features=no_unsafe
