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

use crate::browser::css::selector::Pseudo;

use super::cursor::Cur;
use super::element_args::{compound_arg, ident_arg};

/* :lang(ident), :dir(ltr|rtl), :state(ident), :host(compound) and
 * :host-context(compound). No shadow tree or custom state exists here, so
 * the last three never match; their argument is still checked. */
pub(super) fn argument(c: &mut Cur, name: &str) -> Option<Pseudo> {
    Some(match name {
        "lang" => Pseudo::Lang(ident_arg(c)?.to_ascii_lowercase()),
        "dir" => match ident_arg(c)?.to_ascii_lowercase().as_str() {
            "rtl" => Pseudo::Dir(true),
            "ltr" => Pseudo::Dir(false),
            _ => Pseudo::Never,
        },
        "state" => ident_arg(c).map(|_| Pseudo::Never)?,
        _ => compound_arg(c).map(|_| Pseudo::Never)?,
    })
}
