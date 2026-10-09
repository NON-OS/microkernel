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

use core::sync::atomic::{AtomicBool, Ordering};

use nonos_policy_client::get_bool;
use nonos_policy_proto::Field;

use crate::state::NotifyLevel;

static ENABLED: AtomicBool = AtomicBool::new(true);

// Follow the Notifications setting; an unanswered read keeps the last value.
pub fn follow(port: u32) {
    if port == 0 {
        return;
    }
    if let Some(v) = get_bool(port, Field::NotificationsEnabled) {
        ENABLED.store(v, Ordering::Relaxed);
    }
}

// With notifications off, an app's news is dropped; warnings and errors still show.
pub fn shows(level: NotifyLevel) -> bool {
    ENABLED.load(Ordering::Relaxed) || !matches!(level, NotifyLevel::Info)
}
