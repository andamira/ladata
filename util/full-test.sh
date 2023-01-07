#!/bin/sh

# runs the tests for all 4 combinations of std & unsafe
# stops at the first one that fails

echo ">>> t" && cargo t && \
echo ">>> t safe" && cargo t --features=safe && \
echo ">>> t no-default" && cargo t --no-default-features && \
echo ">>> t no-default safe" && cargo t --no-default-features --features=safe
