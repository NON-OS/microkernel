// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The words of the question a desktop Delete asks: the item's name, what
//! is lost, and Delete and Cancel (render/prompt_buttons.rs).

use alloc::format;

use super::prompt_buttons::DELETE;
use super::prompts::Words;
use crate::state::Context;

const FILE_NOTE: &str =
    "The file is removed from your desktop. There is no Trash to get it back from.";
const FOLDER_NOTE: &str =
    "The folder and everything in it are removed. There is no Trash to get them back from.";

pub(super) fn words(ctx: &Context) -> Option<Words> {
    let target = ctx.pending_delete.target()?;
    Some(Words {
        title: format!("Delete \"{}\"?", target.name),
        body: (if target.is_dir { FOLDER_NOTE } else { FILE_NOTE }).into(),
        buttons: DELETE,
        warn_cut: None,
    })
}
