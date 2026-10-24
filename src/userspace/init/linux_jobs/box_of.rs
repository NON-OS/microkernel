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
//! bytes, Err when the market does not vouch for the policy or cannot say.

use alloc::boxed::Box;

use crate::security::market_capsule::client::linux_policy_wire::{LinuxPolicyReply, MARKET_ARG_LEN};
use crate::security::market_capsule::client::queued_get_linux_policy;

pub(super) type Arg = Box<[u8; MARKET_ARG_LEN]>;

pub(super) fn box_of(listing: &str, release: &str) -> Result<Option<Arg>, ()> {
    match queued_get_linux_policy(listing, release) {
        Ok(LinuxPolicyReply::NoPolicy) => Ok(None),
        Ok(LinuxPolicyReply::Policy(m)) => Ok(Some(m)),
        Ok(LinuxPolicyReply::Refused) | Err(_) => Err(()),
    }
}
