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

use alloc::string::String;

use crate::browser::css::selector::{Pseudo, Simple};

use super::spec::Spec;

/* A compound as parsed: its simple selectors, what they add to the
 * specificity, and the pseudo-element code when one ends the compound. */
pub(super) struct Compound {
    pub simple: Simple,
    pub spec: Spec,
    pub element: u8,
}

/* Two different ids in one compound can never both hold. */
pub(super) fn id(s: &mut Simple, name: String) {
    match &s.id {
        Some(have) if *have != name => s.pseudo.push(Pseudo::Never),
        Some(_) => {}
        None => s.id = Some(name),
    }
}
