// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Host-side harness for input line classification. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/syntax_host.rs -o /tmp/syntax_host && /tmp/syntax_host
//
// The theme is stubbed because only the classification is under test; the
// module is otherwise the shipping one, pulled in by path.

#[path = "syntax_host/bounds.rs"]
mod bounds;
#[path = "syntax_host/mounts.rs"]
mod mounts;
#[path = "syntax_host/quotes.rs"]
mod quotes;
#[path = "syntax_host/support.rs"]
mod support;
#[path = "syntax_host/words.rs"]
mod words;

use mounts::{syntax, term};
