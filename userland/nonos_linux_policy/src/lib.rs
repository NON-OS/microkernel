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

//! The least-privilege box of one Linux program: what capsule_linux serves
//! it, which system calls it may complete, and the identity both are bound
//! to.
//!
//! A policy is 256 canonical bytes. Its hash is what a publisher signs in a
//! release and what the isolation proof is checked against, so `decode`
//! accepts exactly the bytes `encode` writes and nothing else: one policy,
//! one encoding, one hash.

#![cfg_attr(not(test), no_std)]

mod bucket;
mod decode;
pub mod dests;
mod encode;
mod error;
mod identity;
mod layout;
pub mod market;
pub mod mask;
mod policy;
mod syscall_set;
mod unlisted;
pub mod vendor;

pub use bucket::{bucket, BUCKETS};
pub use decode::decode;
pub use error::PolicyError;
pub use identity::{app_id, policy_hash};
pub use layout::{LEN, MAGIC, VERSION};
pub use market::market_ready;
pub use policy::LinuxAppPolicy;
pub use syscall_set::{SyscallSet, SET_LEN};
pub use unlisted::{unlisted, UNLISTED_MASK};

#[cfg(test)]
mod tests;
