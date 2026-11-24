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

//! Which verification put a measurement in a family's list. Kept beside the
//! list and never in the record: M, the list MkSealRead gives and the seal
//! are what they were, and only MkFamilyFact reads a kind.
//!
//! The partition is the two calls that measure, which is all the kernel
//! knows: a local trailer this machine minted (MkLocalVerify), or a
//! publisher's chain (MkCapsuleVerify). A Linux policy is only ever vouched
//! for locally, under its own domain, and this machine mints nothing plain
//! in that domain, so a front asks LOCAL for a policy; a program may be
//! either.

/// Accepted under a local trailer (MkLocalVerify).
pub(crate) const LOCAL: u8 = 1;
/// Accepted under a publisher's chain (MkCapsuleVerify).
pub(crate) const PUBLISHER: u8 = 2;
/// Every kind there is; a mask outside it is no question.
pub(crate) const ALL: u8 = LOCAL | PUBLISHER;
