// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

mod embed;
mod spawn;
mod state;

pub use spawn::{spawn_driver_rtl8822ce_capsule, SpawnError};
pub use state::shared_state;
