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

//! What erasing a particular disk destroys, said in the colour of the loss.

use nonos_app_skeleton::PaintBuffer;
use nonos_blk_client::Contents;

use crate::install::ui::metrics::BODY_PX;
use crate::install::ui::{text, theme};

/// What erasing this particular disk destroys, in the colour of the loss.
pub fn warn(fb: &mut PaintBuffer, x: u32, y: u32, contents: Contents) {
    let (line, colour) = match contents {
        Contents::Nonos => ("holds NØNOS already; it will be replaced", theme::WARN),
        Contents::OtherGpt | Contents::Mbr => {
            ("holds another system; all of it is erased", theme::DANGER)
        }
        Contents::Unknown => {
            ("holds data this installer cannot identify; all of it is erased", theme::DANGER)
        }
        Contents::Blank => ("blank; nothing is lost", theme::MUTED),
    };
    text::line(fb, x, y + 4, line, colour, BODY_PX);
}
