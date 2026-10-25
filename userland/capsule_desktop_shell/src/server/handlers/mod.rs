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

pub mod attention;
pub mod bond_notice;
pub mod brightness_key;
pub mod consent;
pub mod consent_rect;
pub mod delete_prompt;
pub mod dialog_key;
pub mod escape_chord;
pub mod hand_over;
pub mod health;
pub mod input_menu;
pub mod input_menu_geometry;
pub mod installed_launch;
pub mod installed_launch_poll;
pub mod instances;
mod launch_children;
mod launch_find;
mod launch_say;
mod launch_wake;
pub mod launcher_focus;
mod launcher_launch;
pub mod launcher_request;
pub mod launchpad;
pub mod launchpad_key;
mod launchpad_launch;
pub mod lid;
pub mod live_prompt;
mod menu_key;
pub mod menubar_action;
pub mod menubar_click;
pub mod notify;
pub mod focus_child;
pub mod open_with;
pub mod pkg_consent;
pub mod pkg_install;
pub mod power_off;
pub mod relay_table;
pub mod settings_relay;
pub mod spotlight_open;
pub mod spotlight_toggle;
pub mod system_keys;
pub mod take_open_arg;
pub mod tray_register;
pub mod tray_remove;
pub mod tray_update;

pub(super) fn u32_at(buf: &[u8], off: usize) -> Option<u32> {
    let bytes = buf.get(off..off + 4)?;
    Some(u32::from_le_bytes(bytes.try_into().ok()?))
}
