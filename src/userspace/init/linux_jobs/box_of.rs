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

//! The box a release is sold with, as the market answers OP_GET_LINUX_POLICY
//! (I-C5): Ok(None) for a release with no policy of its own (a vendor
//! program, whose box is the vendor table's), Ok(Some) for argv m's 320
//! bytes, Err when the market does not vouch for the policy or cannot say,
//! each said apart.

use alloc::boxed::Box;
use alloc::format;

use crate::security::market_capsule::client::linux_policy_wire::{
    held_to, LinuxPolicyReply, MARKET_ARG_LEN,
};
use crate::security::market_capsule::client::queued_get_linux_policy;
use crate::sys::serial::println;

pub(super) type Arg = Box<[u8; MARKET_ARG_LEN]>;

/// `pin` is the package hash init was given for this release: an answer made
/// from another, as when the index was replaced between asks, is refused.
pub(super) fn box_of(listing: &str, release: &str, pin: &[u8; 32]) -> Result<Option<Arg>, ()> {
    let said = match queued_get_linux_policy(listing, release).map(|answer| held_to(answer, pin)) {
        Ok(None) => format!("the market answered for another {release}; the index changed"),
        Ok(Some(LinuxPolicyReply::NoPolicy)) => return Ok(None),
        Ok(Some(LinuxPolicyReply::Policy(m))) => return Ok(Some(m)),
        Ok(Some(LinuxPolicyReply::Refused)) => {
            format!("the market refuses the box {release} names")
        }
        // Which failure, so a market that cannot answer is not read as a refusal.
        Err(e) => format!("the market could not say its box: {e:?}"),
    };
    println(format!("[LINUX-INSTALL] {said}").as_bytes());
    Err(())
}
