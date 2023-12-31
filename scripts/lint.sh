#!/usr/bin/env sh

cargo clippy -- \
            -Dwarnings \
            -Wclippy::all \
            -Wclippy::pedantic \
            -Aclippy::comparison_chain \
            -Aclippy::missing-panics-doc \
            -Aclippy::uninlined_format_args \
            -Aclippy::module-name-repetitions \
            -Aclippy::redundant-closure-for-method-calls
