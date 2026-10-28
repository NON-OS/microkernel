// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::services::lifecycle::CapsuleState;

static STATE: CapsuleState = CapsuleState::new();

pub(super) fn set_alive(pid: u32) {
    STATE.set_alive(pid);
}

pub fn shared_state() -> &'static CapsuleState {
    &STATE
}
