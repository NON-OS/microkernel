/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* The answers and record the apps tests build on, shared by path. */

use nonos_policy_proto::setup_record::{Answers, Host, Name, Record, Tier};
use nonos_policy_proto::wallpapers_kept::ALL;

pub fn kept() -> Answers {
    Answers {
        keyboard_layout: 2,
        timezone: -5,
        wallpaper: 3,
        username: Name::new(b"ada").unwrap(),
        qwen_tier: Tier::new(b"small").unwrap(),
    }
}

/* The current record of `kept()` with `apps_off` turned off. */
pub fn record(apps_off: u8) -> Record {
    Record { answers: kept(), apps_off, hostname: Host::EMPTY, route: 0, wallpapers_kept: ALL }
}
