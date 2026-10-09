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

use crate::settings::schema::rows::{Block, Live, Pill, Row};

pub const UPDATES: &[Block] = &[Block {
    title: "System image",
    // No "Signed" badge: Settings cannot check the image's signature, and a
    // badge it cannot back would only repeat what the build claimed.
    note: Some("Recorded when this image was built."),
    pill: Pill::None,
    rows: &[
        Row::Live("Version", Live::Version),
        Row::Live("Commit", Live::Commit),
        Row::Live("Toolchain", Live::Toolchain),
        Row::Live("Architecture", Live::Architecture),
    ],
}];
