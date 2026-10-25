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

//! Function 7: a device's firmware label, a package of an instance number
//! and a label string (recalled, unverified), for the inventory.

use super::call::ask;
use crate::load::Aml;
use crate::namespace::Id;
use crate::object::Object;
use crate::platform::{integer, Eval};
use alloc::vec::Vec;

/// Longest label kept; the inventory line has room for no more.
pub const MAX_LABEL: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Label {
    pub instance: u64,
    pub text: Vec<u8>,
}

pub fn device_label(aml: &mut Aml, eval: &mut Eval<'_>, dev: Id) -> Option<Label> {
    let Object::Package(p) = ask(aml, eval, dev, 7)? else { return None };
    let e = p.borrow();
    let instance = e.first().and_then(integer)?;
    let Some(Object::String(s)) = e.get(1) else { return None };
    let text: Vec<u8> = s.borrow().iter().copied().take(MAX_LABEL).collect();
    (!text.is_empty()).then_some(Label { instance, text })
}
