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

/* Why a meter job did not go, in words a person can act on. */

use nonos_payment_proto::{EBUSY, EGAS, ENOENT, ETIME};

use super::JobKind;

pub(super) fn failed(kind: JobKind, status: i32) -> &'static str {
    match (kind, status) {
        (JobKind::Claim, ENOENT) => "Nothing waits to be claimed, or the meter restarted; refresh.",
        (_, ENOENT) => "The meter restarted, so this job's answer is gone. Refresh: the channel shows whether it went through.",
        (_, EBUSY) => "The meter is busy with another job; try again soon.",
        (_, EGAS) => "The meter's ETH is short of gas, or the fees are above its caps.",
        (JobKind::Close, ETIME) => "This channel cannot be closed before its expiry plus the contract's grace.",
        _ => "The meter could not send it.",
    }
}
