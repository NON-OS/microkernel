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

use uefi::prelude::*;

use super::entries::ENTRIES;
use super::input::poll;
use super::intro::intro;
use super::keys::{apply, default_index};
use super::nav::Nav;
use super::render::render;
use super::state::{Dirty, Frame};
use crate::display::gop::is_initialized;
use crate::hardware::HardwareInfo;
use crate::menu::MenuAction;
use crate::security::SecurityContext;

const TIMEOUT_S: u32 = 10;
const TICK_US: usize = 50_000;
const TICKS_PER_S: u32 = 20;

pub fn run(st: &mut SystemTable<Boot>, sec: &SecurityContext, _hw: &HardwareInfo) -> MenuAction {
    if !is_initialized() {
        return MenuAction::Timeout;
    }
    let default = default_index();
    let mut f = Frame::new(default, TIMEOUT_S, sec);
    render(&f, Dirty::All);
    intro(st.boot_services());
    f.intro = false;
    let (mut ticks, mut counting, mut dirty) = (0u32, true, Some(Dirty::All));
    loop {
        if counting {
            let left = TIMEOUT_S.saturating_sub(ticks / TICKS_PER_S);
            if left == 0 {
                return ENTRIES[default].action;
            }
            if left != f.remaining_s {
                f.remaining_s = left;
                dirty = dirty.or(Some(Dirty::Timer));
            }
        }
        if let Some(d) = dirty.take() {
            render(&f, d);
        }
        let before = f.sel;
        let nav = poll(st.boot_services());
        if let Some(action) = apply(nav, &mut f.sel) {
            return action;
        }
        if counting && !matches!(nav, Nav::None) {
            counting = false;
            f.remaining_s = 0;
            dirty = dirty.or(Some(Dirty::Timer));
        }
        if !matches!(dirty, Some(Dirty::All)) && f.sel != before {
            dirty = Some(Dirty::Selection);
        }
        st.boot_services().stall(TICK_US);
        ticks += 1;
    }
}
