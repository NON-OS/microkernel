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

//! A network that came up and found no exit, or did not come up.

use super::{said, Reason};

/// Through Nym, or Anyone, every try in a row found no exit that answered:
/// another try may find one, and a direct download is the other choice.
pub const WHY_NYM_NO_EXIT: u8 = 31;
pub const WHY_ANYONE_NO_EXIT: u8 = 32;
/// The Anyone network, which downloads installs, did not come up in the
/// three minutes a download waits for it.
pub const WHY_ANYONE_NOT_UP: u8 = 33;

pub(super) fn exits_reason(code: u8) -> Option<Reason> {
    Some(match code {
        WHY_NYM_NO_EXIT => said(
            "No Nym exit answered; try again, or press d to download direct (the mirror sees \
             this machine's address)",
            true,
        ),
        WHY_ANYONE_NO_EXIT => said(
            "No Anyone circuit reached the mirror; try again, or press d to download direct \
             (the mirror sees this machine's address)",
            true,
        ),
        WHY_ANYONE_NOT_UP => said(
            "Anyone did not build a circuit within 3 minutes; retry, or press d to download \
             direct (the mirror sees this machine's address)",
            true,
        ),
        _ => return None,
    })
}
