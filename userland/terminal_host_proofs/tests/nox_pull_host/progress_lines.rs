// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! Per-file progress lines and the closing summary.

use crate::progress;

#[test]
fn progress_file_line_has_name_and_size() {
    let l = progress::file_line(b"/docs/report.pdf", 4096);
    assert_eq!(l, b"/docs/report.pdf  4096 bytes");
}

#[test]
fn progress_summary_counts() {
    let t = progress::Tally { ok: 3, skipped: 1, failed: 2 };
    assert_eq!(progress::summary(&t), b"3 ok, 1 skipped, 2 failed");
}
