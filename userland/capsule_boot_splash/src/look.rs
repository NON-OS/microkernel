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

//! What every repaint of the splash shows besides its status line: the
//! attestation status and its verdict, the firmware's logo when it left
//! one, and the layout around it.

use nonos_libc::AttestStatus;

use crate::layout_fit::Layout;
use crate::logo::Shown;

pub(crate) struct Look {
    pub att: AttestStatus,
    pub badge: Option<bool>,
    pub logo: Option<Shown>,
    /// Where the panel and status line sit around that logo.
    pub fit: Layout,
}
