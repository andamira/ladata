#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

MSRV="1.65.0"
RCMD="rustup -v run $MSRV"

# NOTE: need to install thumbv7m-none-eabi
rustup override set $MSRV
rustup target add thumbv7m-none-eabi
# rustup override set stable

cmd="cargo c"; echo "std, safe\n$ " $cmd; $cmd
cmd="cargo cu"; echo "std, unsafe\n$" $cmd; $cmd
cmd="cargo cn"; echo "no-std, safe\n$" $cmd; $cmd
cmd="cargo cNu"; echo "no-std, no-alloc, unsafe\n$" $cmd; $cmd

cmd="cargo t"; echo "tests\n$" $cmd; $cmd
cmd="cargo tu"; echo "tests\n$" $cmd; $cmd

cmd="cargo +nightly nd"; echo "docs\n$" $cmd; $cmd
