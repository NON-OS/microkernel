// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Which flags need a stat.

use crate::support::run;

#[test]
fn needs_meta_tracks_the_flags_that_require_a_stat() {
    assert!(!run(&[b"ls", b"-ah1"]).0.needs_meta());
    for arg in [&b"-l"[..], &b"-t"[..], &b"-S"[..]] {
        assert!(run(&[b"ls", arg]).0.needs_meta(), "{:?}", arg);
    }
}
