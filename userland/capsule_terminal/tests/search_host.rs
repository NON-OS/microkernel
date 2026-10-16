// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Host-side harness for reverse history search. Compiled with the host
// toolchain:
//   rustc --edition 2021 --test tests/search_host.rs -o /tmp/search_host && /tmp/search_host
//
// The substring rule is the shipping one, pulled in by path.

#[path = "search_host/edges.rs"]
mod edges;
#[path = "search_host/finding.rs"]
mod finding;
#[path = "search_host/mounts.rs"]
mod mounts;
#[path = "search_host/support.rs"]
mod support;

use mounts::types;
