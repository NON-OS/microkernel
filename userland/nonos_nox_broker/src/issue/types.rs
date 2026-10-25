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

use nonos_nox_license::Entitlement;
use nonos_nox_receipt::ReceiptError;

/// Where a payment must have gone to count: the payee the signed listing names
/// and the token contract `nonos_nox_config` names. The broker holds neither,
/// so a new publisher or a redeploy never means a new build of it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PaidTo {
    pub payee: [u8; 20],
    pub token: [u8; 20],
}

/// The grant's own terms, the broker's to choose: `now` and `ttl_secs` set
/// the validity window (a zero `ttl_secs` never expires); `device` binds the
/// grant to one install (all zero leaves it unbound).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Terms {
    pub now: u64,
    pub ttl_secs: u64,
    pub nonce: [u8; 8],
    pub device: [u8; 32],
}

/// A granted entitlement, not yet signed, with the terms the broker settled on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Issued {
    /// The grant to sign and return to the buyer.
    pub entitlement: Entitlement,
    /// Whether recording the funding hash evicted an older one, i.e. the
    /// spent-set is at capacity and should be widened.
    pub spent_set_full: bool,
}

/// Why the broker declined to issue.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IssueError {
    /// The tool id is not one we sell, so it has no price.
    UnknownTool,
    /// The payee or the token is the zero address. A transfer there is a burn,
    /// and a listing that never named a payee must not sell anything.
    NoPayee,
    /// The receipt does not prove a sufficient payment to the payee.
    Payment(ReceiptError),
    /// This transaction was already redeemed for a grant.
    Replay,
}
