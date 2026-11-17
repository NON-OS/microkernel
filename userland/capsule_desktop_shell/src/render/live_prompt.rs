// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The words of the offer to install NONOS at the start of a live session:
//! that nothing here is kept, with Install NONOS and Not now.

use super::prompt_buttons::INSTALL;
use super::prompts::Words;

const TITLE: &str = "This is a live session";
const NOTE: &str = "Nothing here is kept: files, settings and models are gone when the machine stops. Install NONOS to keep them.";

pub(super) fn words() -> Words {
    Words {
        title: TITLE.into(),
        body: NOTE.into(),
        buttons: INSTALL,
        warn_cut: None,
    }
}
