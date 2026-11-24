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

//! What the kernel says about a sender and its guest family, which a front
//! asks of it (MkFamilyFact, and MkCapCheck for Network). Behind a trait so
//! the rule is held on the host to answers a test chooses.

use alloc::vec::Vec;

/// Accepted under a local trailer this machine minted (MkLocalVerify): the
/// only kind a vouched policy is (nonos_linux_policy::vouch).
pub const LOCAL: u8 = 1;
/// Accepted under a publisher's chain (MkCapsuleVerify).
pub const PUBLISHER: u8 = 2;
/// A program proved either way.
pub const PROVED: u8 = LOCAL | PUBLISHER;

pub trait Facts {
    /// The sender holds Network itself, and is served as it always was.
    fn holds_network(&self, pid: u32) -> bool;
    /// The live family `pid` supervises has `measurement` among those the
    /// kernel recorded for it, under one of `kinds` (LOCAL, PUBLISHER).
    fn holds(&self, pid: u32, measurement: &[u8; 32], kinds: u8) -> bool;
    /// The generation of the live family `pid` supervises, 0 for none: a
    /// family sealed and the next begun under the same pid differ here.
    fn generation(&self, pid: u32) -> u64;
    /// `pid`'s argv as the kernel set it at spawn, NUL-separated; None when
    /// the kernel would not say. Read only to find a box's destinations, and
    /// never logged or kept (by_box.rs).
    fn argv(&self, pid: u32) -> Option<Vec<u8>>;
}
