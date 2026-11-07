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

use super::display_watch::DisplayWatch;
use super::{Restack, SubscriptionList};
use crate::focus::owed_keys::OwedKeys;
use crate::focus::FocusModel;
use crate::window::WindowTable;
use crate::z_order::ZStack;

pub struct Context {
    pub compositor_port: u32,
    pub display_width: u32,
    pub display_height: u32,
    pub windows: WindowTable,
    pub focus: FocusModel,
    pub z: ZStack,
    pub subscriptions: SubscriptionList,
    pub next_request_id: u32,
    pub input_router_pid: u32,
    /// Whether the compositor is owed the stack again (restack.rs).
    pub restack: Restack,
    /// KEYS notices an owner's full inbox has not taken yet (focus/owed_keys.rs).
    pub owed_keys: OwedKeys,
    /// When the display size was last read (display_watch.rs).
    pub display: DisplayWatch,
}

impl Context {
    pub fn issue_request_id(&mut self) -> u32 {
        let id = self.next_request_id;
        self.next_request_id = id.wrapping_add(1).max(1);
        id
    }
}
