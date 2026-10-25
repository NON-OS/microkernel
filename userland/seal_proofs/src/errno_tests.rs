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

//! The kernel's errno table, held to the Linux errno each value is named
//! for, negated, so a renumbered or a duplicated value is caught here.

use super::kernel::*;

#[test]
fn every_errno_is_its_linux_number_negated() {
    let table = [
        (ERRNO_PERM, 1),
        (ERRNO_NOENT, 2),
        (ERRNO_SRCH, 3),
        (ERRNO_IO, 5),
        (ERRNO_CHILD, 10),
        (ERRNO_AGAIN, 11),
        (ERRNO_NOMEM, 12),
        (ERRNO_ACCES, 13),
        (ERRNO_FAULT, 14),
        (ERRNO_BUSY, 16),
        (ERRNO_EXIST, 17),
        (ERRNO_XDEV, 18),
        (ERRNO_NODEV, 19),
        (ERRNO_INVAL, 22),
        (ERRNO_NOTTY, 25),
        (ERRNO_FBIG, 27),
        (ERRNO_NOSPC, 28),
        (ERRNO_RANGE, 34),
        (ERRNO_NOSYS, 38),
        (ERRNO_BADMSG, 74),
        (ERRNO_NOTSUP, 95),
        (ERRNO_NETDOWN, 100),
        (ERRNO_TIMEDOUT, 110),
        (ERRNO_ALREADY, 114),
        (ERRNO_INPROGRESS, 115),
        (ERRNO_STALE, 116),
    ];
    for (value, linux) in table {
        assert_eq!(value, -linux);
    }
}
