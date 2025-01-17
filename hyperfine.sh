hyperfine --warmup 5 'cargo test -r -F pext perft -- --include-ignored' 'cargo test -r -F magic perft -- --include-ignored' 'cargo test -r perft -- --include-ignored'
