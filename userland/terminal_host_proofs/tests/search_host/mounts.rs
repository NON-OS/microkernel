// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! A plain history store and the shipping reverse search, mounted by path.

const COLS: usize = 256;

const HISTORY_DEPTH: usize = 64;

pub mod types {
    use super::{COLS, HISTORY_DEPTH};
    pub struct History {
        pub(super) entries: [[u8; COLS]; HISTORY_DEPTH],
        pub(super) lengths: [usize; HISTORY_DEPTH],
        pub(super) count: usize,
        pub(super) cursor: Option<usize>,
    }
    impl History {
        pub fn new() -> Self {
            Self {
                entries: [[0; COLS]; HISTORY_DEPTH],
                lengths: [0; HISTORY_DEPTH],
                count: 0,
                cursor: None,
            }
        }
        pub fn push(&mut self, line: &[u8]) {
            let n = line.len().min(COLS);
            self.entries[self.count][..n].copy_from_slice(&line[..n]);
            self.lengths[self.count] = n;
            self.count += 1;
            let _ = self.cursor;
        }
    }
}

#[path = "../../../capsule_terminal/src/term/history/search_back.rs"]
pub mod search_back;
