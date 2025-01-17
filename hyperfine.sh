perft_features() {
    printf "cargo test -r %s perft -- --include-ignored" "${1}"
}

hyperfine --warmup 5\
    "$(perft_features "-F pext")"\
    "$(perft_features "-F magic")"\
    "$(perft_features)"\
    "$(perft_features "-F inline")"\
    "$(perft_features "-F inline-aggressive")"\
    "$(perft_features "-F pext,inline")"\
    "$(perft_features "-F pext,inline-aggressive")"\
    "$(perft_features "-F magic,inline")"\
    "$(perft_features "-F magic,inline-aggressive")"\
