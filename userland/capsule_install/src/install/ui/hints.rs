/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * The keys that do something on this screen, and nothing else: a person
 * should never have to guess what Enter does on an installer. Full screen,
 * leaving starts the desktop, and after the install only a restart is
 * offered, since nothing else runs until the machine boots its new disk.
 */

use crate::install::event::stoppable;
use crate::install::full::active;
use crate::install::state::{Screen, State};

pub fn hints(state: &State) -> (&'static str, &'static str) {
    let leave =
        if active() { "Esc  leave the installer; the desktop starts" } else { "Esc  close" };
    match state.screen {
        Screen::Welcome if state.image.is_some() => (leave, "Enter  choose a disk"),
        Screen::Welcome => (leave, ""),
        Screen::Disks => ("Esc  back", "Up/Down  select    Enter  continue"),
        Screen::Confirm if matches!(state.prepared, Some(Ok(_))) => {
            ("Esc  back", "type the word, then Enter")
        }
        Screen::Confirm => ("Esc  back", ""),
        Screen::Writing if stoppable(state) => {
            ("Esc  stop (disk left without a table)", "do not power off")
        }
        Screen::Writing => ("", "writing the partition table; do not power off"),
        Screen::Verifying => ("", "do not power off"),
        Screen::Done if active() => ("", "Enter  restart now"),
        Screen::Done => ("Esc  close", "Enter  restart now"),
        Screen::Failed => (leave, "Enter  choose another disk"),
    }
}
