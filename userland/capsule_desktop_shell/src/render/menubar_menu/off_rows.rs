/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The menu rows that open an app setup can turn off, as the dispatcher in
 * server/handlers/menubar_action.rs maps them. A row whose app is off is
 * drawn dim, and the dispatcher opens nothing for it.
 */

use crate::apps_off::is_off;
use crate::render::palette;

fn service(title: usize, row: usize) -> Option<&'static [u8]> {
    match (title, row) {
        (1, 2) => Some(b"app.file_manager"),
        (3, 1) => Some(b"app.browser"),
        _ => None,
    }
}

/* The colour a row's label is drawn in. */
pub(super) fn row_fg(title: usize, row: usize) -> u32 {
    match service(title, row) {
        Some(s) if is_off(s) => palette::TEXT_MUTED,
        _ => palette::TEXT,
    }
}
