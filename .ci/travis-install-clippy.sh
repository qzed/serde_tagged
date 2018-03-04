#!/usr/bin/env sh
set -e

if [ "$(cargo +$NIGHTLY clippy -- --version)" != "$CLIPPY" ] ; then
    echo "Installing clippy '$CLIPPY' for rust nightly '$NIGHTLY'"
    cargo +$NIGHTLY install clippy --force --vers $CLIPPY
else
    echo "Using cached clippy '$CLIPPY' for rust nightly '$NIGHTLY'"
fi
