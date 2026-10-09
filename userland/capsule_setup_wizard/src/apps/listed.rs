/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The optional apps setup lists: those the kernel says it carries, in the
 * order nonos_policy_proto::apps has them.
 */

use nonos_policy_proto::apps::{App, OPTIONAL};

use crate::setup::machine::header;

/* The procstat layout that first carried the app switches. */
const APPS_VERSION: u32 = 4;

/* The switches whose apps this image carries; none when the kernel would not say. */
pub fn present() -> u8 {
    header().filter(|h| h.version >= APPS_VERSION).map_or(0, |h| h.apps_present as u8)
}

pub fn listed(present: u8) -> impl Iterator<Item = &'static App> {
    OPTIONAL.iter().filter(move |a| present & a.bit != 0)
}
