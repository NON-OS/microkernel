/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* One line under each screen's title: what the screen is for. */

use crate::install::state::Screen;

pub(super) fn subtitle(screen: Screen) -> &'static str {
    match screen {
        Screen::Welcome => "What this computer booted, and what it has, read from the machine.",
        Screen::Disks => "The disks this boot's drivers serve. The one you choose is erased.",
        Screen::Confirm => "The plan for this disk. Nothing is written until the word is typed.",
        Screen::Writing => "Writing the new disk. Keep the computer on.",
        Screen::Verifying => "Reading every sector back. Keep the computer on.",
        Screen::Done => "Every sector read back as written.",
        Screen::Failed => "The install stopped.",
    }
}
