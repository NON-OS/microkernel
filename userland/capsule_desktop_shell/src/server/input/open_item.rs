// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A desktop item opened in the app that suits it, told which item.

use crate::server::desktop;
use crate::server::handlers::launcher_request::LaunchOutcome;
use crate::state::{Context, LAUNCHER_APPS};

pub(super) fn is_image_name(name: &str) -> bool {
    let Some((_, ext)) = name.rsplit_once('.') else { return false };
    ext.eq_ignore_ascii_case("png")
        || ext.eq_ignore_ascii_case("jpg")
        || ext.eq_ignore_ascii_case("jpeg")
        || ext.eq_ignore_ascii_case("bmp")
        || ext.eq_ignore_ascii_case("gif")
}

// Open a desktop item in the app that suits it, and tell that app which item.
// This used to launch the file manager or the text editor and hand over
// nothing, so every icon opened the same blank app at its default location.
// The path travels the way the file manager's Open With already sends it.
pub(super) fn open_item(ctx: &mut Context, idx: usize) {
    let Some(item) = ctx.desktop_items.get(idx) else { return };
    // A picture opens in the image viewer, which decodes each kind named in
    // is_image_name; the editor would show a screenful of bytes.
    let service: &[u8] = if item.is_dir {
        b"app.file_manager"
    } else if is_image_name(&item.name) {
        b"app.image_viewer"
    } else {
        b"app.text_editor"
    };
    // By service rather than by position: the table is edited often enough
    // that an index would drift into launching the wrong app.
    let Some(index) = LAUNCHER_APPS.iter().position(|a| a.service == service) else { return };
    // Desktop items are the home listing, so the path is the name under it.
    // Built from the one definition the listing uses, or an icon would open a
    // path that is not the file it was drawn from.
    let Some(path) = desktop::home_path(&item.name) else { return };
    if super::super::handlers::hand_over::hand_over(ctx, index, path) == LaunchOutcome::Failed {
        crate::apps_off::toast_failed(ctx, service, crate::server::toast_clock::now());
    }
}
