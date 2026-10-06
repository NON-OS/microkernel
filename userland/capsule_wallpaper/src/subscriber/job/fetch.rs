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

//! One job: fetch wallpaper `index` from the catalog, going on from `kept`
//! when that is what came over of it before, and decode it. Every call it
//! makes waits on the catalog, up to 15 s for the first chunk of a try, and
//! the decode takes a while too, so this runs on the worker, never on the
//! thread that answers the service.

use super::plan::Outcome;
use crate::catalog_client::Download;
use crate::paint::{decode_jpeg, DecodedImage};

pub fn fetch(
    catalog_port: u32,
    index: u8,
    kept: Option<Download>,
) -> Outcome<Download, DecodedImage> {
    let kept = kept.filter(|d| d.index() == index);
    let mut download = match kept.or_else(|| Download::start(catalog_port, index)) {
        Some(download) => download,
        None => return Outcome::Failed("asking the catalog its size"),
    };
    if !download.resume(catalog_port) {
        return Outcome::Stopped(download);
    }
    let bytes = download.into_bytes();
    match decode_jpeg(&bytes) {
        Some(image) => Outcome::Decoded(image),
        None => Outcome::Failed("decoding it"),
    }
}
