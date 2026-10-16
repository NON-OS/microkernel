// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
//
// Host-side harness for the `ls` argument parser. Compiled with the host
// toolchain so clustering, `--`, repeats and unknown options are exercised
// without QEMU:
//   rustc --edition 2021 --test tests/lsflags_host.rs -o /tmp/lsflags_host && /tmp/lsflags_host
//
// The module is pulled in by path so this cannot pass against a copy that has
// drifted from what the capsule runs.

extern crate alloc;

#[path = "lsflags_host/clusters.rs"]
mod clusters;
#[path = "lsflags_host/meta.rs"]
mod meta;
#[path = "lsflags_host/mounts.rs"]
mod mounts;
#[path = "lsflags_host/operands.rs"]
mod operands;
#[path = "lsflags_host/support.rs"]
mod support;
#[path = "lsflags_host/unknown.rs"]
mod unknown;

use mounts::ls_flags;
