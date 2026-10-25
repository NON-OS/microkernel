// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! The most a claimant's inboxes hold before it reads them, read from the
//! kernel (L80): a capsule owns `proc.<pid>` at the default capacity and
//! `stdin.<pid>` at its own from its first moment, each registration pinned
//! whole (whole), and the default the constant itself.

use super::code::flat;
use super::whole::{num, whole};

const INBOX: &str = include_str!("../../../../../src/ipc/nonos_inbox/registry.rs");
const OWN: &str = include_str!(
    "../../../../../src/kernel_core/process_spawn/capsule_spawn/runner/install/own_inboxes.rs"
);

const OWNS: [&str; 5] = [
    "pub(super) fn register(pid: u32) -> Result<(), SpawnError> {",
    r#"nonos_inbox::register_inbox(&format!("proc.{}", pid), pid)"#,
    ".map_err(|_| SpawnError::ProcessCreation)?;",
    r#"nonos_inbox::register_inbox_with_capacity(&format!("stdin.{}", pid), pid, STDIN_CAPACITY)"#,
    ".map_err(|_| SpawnError::ProcessCreation) }",
];

const DEFAULT: [&str; 2] = [
    "pub fn register_inbox(module: &str, owner_pid: u32) -> Result<(), InboxError> {",
    "register_inbox_with_capacity(module, owner_pid, DEFAULT_CAP.load(Ordering::Relaxed)) }",
];

/// Messages the claimant's own inboxes hold at most.
pub(super) fn held() -> u64 {
    let (i, o) = (flat(INBOX), flat(OWN));
    whole(&o, "register", &OWNS);
    whole(&i, "register_inbox", &DEFAULT);
    let at_first = "static DEFAULT_CAP: AtomicUsize = AtomicUsize::new(DEFAULT_INBOX_CAPACITY);";
    assert!(i.contains(at_first), "the default capacity is not the constant");
    num::<u64>(&i, "DEFAULT_INBOX_CAPACITY", 2) + num::<u64>(&o, "STDIN_CAPACITY", 2)
}
