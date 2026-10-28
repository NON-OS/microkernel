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

//! `OP_GET_LINUX_POLICY` (I-C5): the box a Linux release is sold with, as
//! init hands it to capsule_linux's install role in argv m.
//!
//! Body: version u8 (2) | kind u8 | package_hash[32] | for KIND_POLICY
//! only, policy[256] | listing_key[32] | release_key[32] | the release's
//! listed arguments and extra packages, 0 to MARKET_ARGS_MAX bytes. The package hash is
//! the release the answer was made from, which init holds to the pin it
//! was given, so an index replaced between its asks is refused, never mixed.
//! Any other version, kind or length is refused, so a reply is exactly one
//! of the three or nothing.

use alloc::boxed::Box;
use alloc::vec::Vec;

mod body;
mod parse;
#[cfg(test)]
mod tests;

pub use body::linux_policy_body;
pub use parse::parse_linux_policy;

pub const LINUX_POLICY_VERSION: u8 = 2;
/// No policy rides with this release (a vendor program, whose box is the
/// vendor table's, or no Linux listing): init sends "-".
pub const KIND_NONE: u8 = 0;
/// A policy the market verified under the release's signature and ids.
pub const KIND_POLICY: u8 = 1;
/// A policy the market does not vouch for: the install is refused.
pub const KIND_REFUSED: u8 = 2;
/// policy[256] | listing_key[32] | release_key[32], argv m's bytes.
pub const MARKET_ARG_LEN: usize = 320;
/// The most bytes of listed arguments after them (nonos_linux_policy::args).
pub const MARKET_ARGS_MAX: usize = 1024;
/// version | kind | package_hash.
pub const LINUX_POLICY_HEAD: usize = 34;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LinuxPolicy {
    NoPolicy,
    /// argv m's bytes, then the listed arguments, empty for none.
    Policy(Box<[u8; MARKET_ARG_LEN]>, Vec<u8>),
    Refused,
}
