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

//! The test modules, each at its own file under src/, grouped out of lib.rs.

#[path = "chord_tests.rs"]
mod chord_tests;
#[path = "damage_tests.rs"]
mod damage_tests;
#[path = "gesture_click_tests.rs"]
mod gesture_click_tests;
#[path = "gesture_tests.rs"]
mod gesture_tests;
#[path = "held_keys_tests.rs"]
mod held_keys_tests;
#[path = "hid_touchpad_tests.rs"]
mod hid_touchpad_tests;
#[path = "key_target_tests.rs"]
mod key_target_tests;
#[path = "press_frame_tests.rs"]
mod press_frame_tests;
#[path = "ps2_tests.rs"]
mod ps2_tests;
#[path = "router_hover_tests.rs"]
mod router_hover_tests;
#[path = "router_key_tests.rs"]
mod router_key_tests;
#[path = "router_parse_tests.rs"]
mod router_parse_tests;
#[path = "router_press_tests.rs"]
mod router_press_tests;
#[path = "router_route_tests.rs"]
mod router_route_tests;
#[path = "router_system_key_tests.rs"]
mod router_system_key_tests;
#[path = "touchpad_tests.rs"]
mod touchpad_tests;
#[path = "usb_keycode_tests.rs"]
mod usb_keycode_tests;
#[path = "usb_repeat_tests.rs"]
mod usb_repeat_tests;
#[path = "layout_tests.rs"]
mod layout_tests;
