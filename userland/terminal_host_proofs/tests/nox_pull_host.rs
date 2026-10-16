// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// Host-side unit harness for the pure nox-pull parsers, built for the host
// (not the capsule target) so the layout-independent parsing logic is
// exercised without QEMU.
//
// Modules are declared flat as siblings in `nox_pull_host/mounts.rs` so
// `super::scan` inside http.rs / recurse.rs resolves to a sibling (mirroring
// `pull::scan` in the capsule); the root re-exports them for `crate::` paths.

extern crate alloc;

#[path = "nox_pull_host/args_flags.rs"]
mod args_flags;
#[path = "nox_pull_host/args_parse.rs"]
mod args_parse;
#[path = "nox_pull_host/autoindex.rs"]
mod autoindex;
#[path = "nox_pull_host/checksums.rs"]
mod checksums;
#[path = "nox_pull_host/codecs.rs"]
mod codecs;
#[path = "nox_pull_host/frames.rs"]
mod frames;
#[path = "nox_pull_host/http_build.rs"]
mod http_build;
#[path = "nox_pull_host/http_parse.rs"]
mod http_parse;
#[path = "nox_pull_host/mounts.rs"]
pub mod mounts;
#[path = "nox_pull_host/mounts_codecs.rs"]
mod mounts_codecs;
#[path = "nox_pull_host/progress_lines.rs"]
mod progress_lines;
#[path = "nox_pull_host/redirects.rs"]
mod redirects;
#[path = "nox_pull_host/scanning.rs"]
mod scanning;
#[path = "nox_pull_host/support.rs"]
mod support;
#[path = "nox_pull_host/targets.rs"]
mod targets;

use mounts::{args, framing, http, progress, recurse, redirect, scan, target, term, verify};
use mounts_codecs::{base64, inflate};
