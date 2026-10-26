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

//! How far the toasts have been brought on screen: whether their layer is
//! live, the queue generations painted and synced, and the panel's window.

pub struct ToastView {
    pub toast_layer_live: bool,
    /// The toast queue's generation the chrome was last painted with.
    pub toasts_drawn: u32,
    /// The toast queue's generation the panel was last brought on screen
    /// with (render/toasts.rs sync_toast_layer). The serve loop syncs the
    /// panel whenever the queue moved past it, whoever pushed or expired.
    pub toasts_synced: u32,
    /// The panel the shell holds a window over for its toasts, as last
    /// opened with the window manager (render/toasts.rs claim_presses).
    pub toast_window: Option<(u32, u32, u32, u32)>,
}

impl Default for ToastView {
    fn default() -> Self {
        Self::new()
    }
}

impl ToastView {
    pub fn new() -> Self {
        ToastView { toast_layer_live: false, toasts_drawn: 0, toasts_synced: 0, toast_window: None }
    }
}
