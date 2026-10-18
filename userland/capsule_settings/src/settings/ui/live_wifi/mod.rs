/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The Wi-Fi page's live rows: the link, the join in progress or its outcome,
//! and whether joins are remembered.

mod join;
mod link;
mod remember;

pub use join::join;
pub use link::link;
pub use remember::remember;
