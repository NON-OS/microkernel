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

//! A Linux personality's guest family, granted the anonymity network by the
//! front that carries its streams, without the personality ever holding
//! Network (userland/capsule_linux/design/network-grant.md).
//!
//! The kernel reports facts (`Facts`, MkFamilyFact); this crate decides:
//! whether a family may have a grant (`decide`), whether a sender is served
//! now (`Grants::admits`), and whether a stream may go where it asks
//! (`Grants::may_reach`). Every grant is bound to one family by the kernel's
//! generation for it, checked on every request.

#![no_std]

extern crate alloc;

mod by_box;
mod by_policy;
mod decide;
mod facts;
mod grant;
#[cfg(feature = "kernel")]
mod kernel_facts;
mod names_destinations;
mod reaches;
mod refusal;
mod table;
pub mod wire;

#[cfg(test)]
mod tests;

pub use by_box::by_box;
pub use by_policy::by_policy;
pub use decide::{decide, Ask};
pub use facts::{Facts, LOCAL, PROVED, PUBLISHER};
pub use grant::Grant;
#[cfg(feature = "kernel")]
pub use kernel_facts::KernelFacts;
pub use names_destinations::{box_word, names_destinations};
pub use reaches::reaches;
pub use refusal::Refusal;
pub use table::{Grants, MAX_ROWS};
