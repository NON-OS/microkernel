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

//! The registers that moved on the RTL8125. Linux keeps the 8168 offsets in
//! `enum rtl_registers` and the moved ones in `enum rtl8125_registers`
//! (r8169_main.c); every access that differs goes through here.

mod doorbell;
mod events;
pub mod layout;

pub use doorbell::ring;
pub use events::{ack, mask_and_ack, read_events, read_mask};
pub use layout::{doorbell_of, events_of};
