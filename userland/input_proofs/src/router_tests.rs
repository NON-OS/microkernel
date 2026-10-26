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

//! The input router's own proofs: focus, holds, the trusted band and its
//! chords, the lid and brightness keys, the touchpad holds, and the bond
//! between the router and the compositor. Each file sits beside lib.rs; a
//! `#[path]` here is relative to src/.

#[path = "router_bond_hold_tests.rs"]
mod router_bond_hold_tests;
#[path = "router_bond_kit.rs"]
pub(crate) mod router_bond_kit;
#[path = "router_bond_tests.rs"]
mod router_bond_tests;
#[path = "router_bond_wire_tests.rs"]
mod router_bond_wire_tests;
#[path = "router_brightness_key_tests.rs"]
mod router_brightness_key_tests;
#[path = "router_display_tests.rs"]
mod router_display_tests;
#[path = "router_focus_tests.rs"]
mod router_focus_tests;
#[path = "router_hold_end_tests.rs"]
mod router_hold_end_tests;
#[path = "router_lid_key_tests.rs"]
pub mod router_lid_key_tests;
#[path = "router_mode_tests.rs"]
mod router_mode_tests;
#[path = "router_pad_hold_tests.rs"]
mod router_pad_hold_tests;
#[path = "router_pad_tests.rs"]
pub(crate) mod router_pad_tests;
#[path = "router_trusted_tests.rs"]
pub(crate) mod router_trusted_tests;
#[path = "secure_chord_tests.rs"]
mod secure_chord_tests;
#[path = "switch_chord_tests.rs"]
mod switch_chord_tests;
#[path = "terminal_chord_tests.rs"]
mod terminal_chord_tests;
#[path = "trusted_rule_tests.rs"]
mod trusted_rule_tests;
