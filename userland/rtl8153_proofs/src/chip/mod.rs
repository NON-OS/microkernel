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

//! A register model of the RTL8153 behind the scripted bus: its register
//! space, the vendor requests answered from it, the few bits the chip
//! clears by itself, and descriptors laid out as r8152 expects them.

mod answer;
mod bound;
mod byte_enable;
mod descriptors;
mod power;
mod regs;

pub use answer::rtl8153;
pub use bound::{bound, found, reads_of, RTL8153_IDS};
pub use power::TEST_MAC;
pub use regs::{PHY, PLA, USB};
