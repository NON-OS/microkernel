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

use crate::setup::prime::{overlay::Overlay, peers::Peers};
use crate::state::{new_taskbar_state, Context, ToastQueue, TrayTable};

pub fn build_context(
    peers: &Peers,
    overlay: &Overlay,
    desk: &Overlay,
    wallpaper_policy_sent: bool,
) -> Context {
    Context {
        ports: crate::state::ctx::ports::Ports {
            compositor_port: peers.compositor_port,
            wm_port: peers.wm_port,
            input_router_port: peers.input_router_port,
            wallpaper_port: peers.wallpaper_port,
            policy_port: 0,
        },
        links: crate::state::ctx::links::Links {
            wallpaper_policy_sent: wallpaper_policy_sent,
            input_kind_mask: super::input_mask::SHELL_INPUT_MASK,
            input_ready: false,
            wm_notify_ready: false,
            grab_held: 0,
        },
        width: overlay.width,
        height: overlay.height,
        scale: crate::state::scale::quarters_for(overlay.width, overlay.height),
        stride: overlay.stride,
        backing_va: overlay.backing_va,
        desk_va: desk.backing_va,
        chrome_va: overlay.backing_va,
        back: alloc::vec::Vec::new(),
        tray: TrayTable::new(),
        taskbar: new_taskbar_state(),
        launchpad_ui: crate::state::ctx::launchpad_ui::LaunchpadUi::new(),
        toasts: ToastQueue::new(),
        toast_view: crate::state::ctx::toast_view::ToastView::new(),
        readings: crate::state::ctx::readings::Readings::new(),
        store_apps: crate::state::ctx::store_apps::StoreApps::new(),
        next_request_id: 2,
        desktop_items: alloc::vec::Vec::new(),
        desk_ui: crate::state::ctx::desk_ui::DeskUi::new(),
        menubar: crate::state::new_menubar_state(),
        pending: crate::state::ctx::pending::Pending::new(),
        trusted: crate::state::trusted::Trusted::default(),
        dialog_focus: crate::state::dialog_keys::DialogFocus::new(),
    }
}
