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

//! Whom the shell gives the focus frame a freshly loaded app waits on before
//! it builds its window (launch_wake.rs), when another process asks: only
//! the asker's own child, and only an app that draws (GraphicsSurfaceCreate).
//! The app takes the frame from the shell alone, so the Terminal that loaded
//! it asks here rather than sending it, and no process can wake another's app.

/// GraphicsSurfaceCreate: a process without it has no window to open.
pub const DRAWS: u64 = 1 << 12;

/// Whether `asker` may have the shell focus the process whose parent is
/// `parent` and whose caps are `caps`.
pub fn may_focus(asker: u32, parent: u32, caps: u64) -> bool {
    asker != 0 && parent == asker && caps & DRAWS != 0
}
