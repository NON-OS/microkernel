// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::handle::{is_console, note_chosen};
use super::order::console_order;
use super::try_init::try_init;
use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::Identify;

pub fn init_gop(st: &mut SystemTable<Boot>) -> bool {
    let bs = st.boot_services();
    match bs.locate_handle_buffer(uefi::table::boot::SearchType::ByProtocol(&GraphicsOutput::GUID))
    {
        Ok(h) => {
            let console: Vec<bool> = h.iter().map(|&hnd| is_console(bs, hnd)).collect();
            for i in console_order(&console) {
                if try_init(bs, h[i]) {
                    note_chosen(&console, i);
                    return true;
                }
            }
            false
        }
        Err(_) => {
            bs.get_handle_for_protocol::<GraphicsOutput>().map(|h| try_init(bs, h)).unwrap_or(false)
        }
    }
}

// A development build can pin the exact mode the firmware should set, e.g.
// "1280x800" so the QEMU window opens at a size that fits the host screen.
// Unset (every hardware build), the picker keeps the panel's native mode.
const PREFERRED_MODE: Option<&str> = option_env!("NONOS_GOP_PREF");

pub(crate) fn preferred_mode() -> Option<(usize, usize)> {
    let s = PREFERRED_MODE?;
    let (w, h) = s.split_once('x')?;
    Some((w.parse().ok()?, h.parse().ok()?))
}
