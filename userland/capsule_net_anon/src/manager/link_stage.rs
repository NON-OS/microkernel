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

//! Which stage of opening a link gave up.

use crate::link::LinkError;

pub(super) fn stage_of(cause: LinkError) -> &'static [u8] {
    match cause {
        LinkError::Connect => b"guard would not take a connection",
        LinkError::Tls => b"guard TLS failed",
        LinkError::Protocol => b"guard broke cell framing",
        LinkError::Version => b"guard shares no link version",
        LinkError::Identity => b"guard is not who it claimed",
        LinkError::Closed => b"guard closed the link",
    }
}
