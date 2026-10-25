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

/*
 * The meter's refusal of a budget, in words a person can act on (B8).
 */

use nonos_payment_proto::{EAGAIN, EINVAL, ENFILE, ENOPERSIST, EPERM};

pub fn budget_refusal(status: i32) -> &'static str {
    match status {
        ENOPERSIST => {
            "This boot keeps nothing past power off, so the meter holds no key and takes no budget."
        }
        EPERM => "The meter takes a budget only from this wallet.",
        EAGAIN => "The meter did not answer. It may still be starting; try again.",
        EINVAL | ENFILE => "The meter refused the budget as written.",
        _ => "The meter could not keep the budget.",
    }
}
