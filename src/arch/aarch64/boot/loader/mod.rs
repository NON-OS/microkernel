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

//! The NONOS loader's handoff, when a NONOS loader started this kernel: its
//! physical address arrives in x1, beside the device tree in x0. Zero is no
//! loader (QEMU's -kernel, U-Boot, a Pi's firmware), and the boot is the
//! device tree's alone, as it always was.

mod install;
mod place;
mod take;

pub(super) use install::install;
pub(super) use place::placed;
pub(super) use take::{take, taken};
