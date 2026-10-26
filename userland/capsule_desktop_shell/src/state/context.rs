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

use super::{MenubarState, TaskbarState, ToastQueue, TrayTable};

pub struct Context {
    /// The services' ports (`ctx/ports.rs`).
    pub ports: crate::state::ctx::ports::Ports,
    /// Where the shell stands with its peers (`ctx/links.rs`).
    pub links: crate::state::ctx::links::Links,
    pub width: u32,
    pub height: u32,
    /// Drawing pixels per logical pixel, in quarters (`state::scale`).
    pub scale: u32,
    pub stride: u32,
    /// The surface every painter draws on. It is the chrome surface, over
    /// the windows, except while the desktop's icons are painted onto the
    /// desk surface under them (render/chrome.rs paint_desk).
    pub backing_va: u64,
    /// The shell's two surfaces, both the size of the display, each kept as
    /// two to flip between (`ctx/flip.rs`): the desk in the compositor's band
    /// under every application window, with the desktop's icons, and the
    /// chrome in the band over every window, with the menubar, the dock,
    /// menus, the Launchpad, toasts and dialogs.
    pub desk: crate::state::ctx::flip::Flip,
    pub chrome: crate::state::ctx::flip::Flip,
    /// The off-screen frame each surface's paint is drawn in before it is
    /// copied over the surface (render/whole_frame.rs), kept between paints.
    pub back: alloc::vec::Vec<u32>,
    pub tray: TrayTable,
    pub taskbar: TaskbarState,
    /// The Launchpad overlay (`ctx/launchpad_ui.rs`).
    pub launchpad_ui: crate::state::ctx::launchpad_ui::LaunchpadUi,
    pub toasts: ToastQueue,
    /// How far the toasts are on screen (`ctx/toast_view.rs`).
    pub toast_view: crate::state::ctx::toast_view::ToastView,
    /// What the shell last read of the system (`ctx/readings.rs`).
    pub readings: crate::state::ctx::readings::Readings,
    /// The capsule store's apps (`ctx/store_apps.rs`).
    pub store_apps: crate::state::ctx::store_apps::StoreApps,
    pub next_request_id: u32,
    /// Entries at the VFS root, shown as icons on the desktop. Loaded lazily
    /// once the vfs_pool service is up, then refreshed after we mutate it.
    pub desktop_items: alloc::vec::Vec<crate::vfs_client::Entry>,
    /// Which menu-bar title is open, and the row under the pointer inside it.
    pub menubar: MenubarState,
    /// The desktop's menu, rename and drag (`ctx/desk_ui.rs`).
    pub desk_ui: crate::state::ctx::desk_ui::DeskUi,
    /// What waits on another process or a person (`ctx/pending.rs`).
    pub pending: crate::state::ctx::pending::Pending,
    /// The chrome's band and the secure attention chord (`state/trusted.rs`).
    pub trusted: crate::state::trusted::Trusted,
    /// Which of the open dialog's two buttons the keyboard is on.
    pub dialog_focus: crate::state::dialog_keys::DialogFocus,
    /// The window switcher's order and run (`ctx/switch`).
    pub switch: crate::state::ctx::switch::SwitchOrder,
    /// The pointer's shape over the chrome (`ctx/pointer_shape.rs`).
    pub pointer_shape: crate::state::ctx::pointer_shape::PointerShape,
}
