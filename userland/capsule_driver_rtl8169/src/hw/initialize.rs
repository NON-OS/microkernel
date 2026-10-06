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

use super::{init_8125, init_8168g};
use crate::chip::MacVersion;
use crate::regs::Regs;

/// Linux rtl_hw_initialize: the per-generation steps taken once at probe,
/// before the first reset. Older chips have none.
pub fn initialize(regs: &Regs, ver: MacVersion) {
    if ver.is_8168g_up() {
        init_8168g(regs, ver);
    } else if ver.is_8125() {
        init_8125(regs, ver);
    }
}
