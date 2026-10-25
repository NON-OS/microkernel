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

//! _CPC (ACPI 6.5, 8.4.6.1), Collaborative Processor Performance Control:
//! abstract levels in place of _PSS's list, each an integer or a register.

use super::cpc_index::FIELDS;
use super::register::{register, Register};
use crate::object::Object;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CpcField {
    /// Missing, of another type, or the null register.
    #[default]
    Absent,
    Integer(u64),
    Register(Register),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cpc {
    pub revision: u8,
    pub fields: [CpcField; FIELDS],
}

impl Cpc {
    /// An entry's value when the package gives it as an integer.
    pub fn integer(&self, i: usize) -> Option<u64> {
        match self.fields.get(i)? {
            CpcField::Integer(v) => Some(*v),
            _ => None,
        }
    }
}

fn field(o: &Object) -> CpcField {
    match o {
        Object::Integer(v) => CpcField::Integer(*v),
        o => register(o).filter(|r| !r.is_null()).map_or(CpcField::Absent, CpcField::Register),
    }
}

/// Entries a revision lacks read as absent. A package naming fewer than
/// 20 elements (through the energy preference register), or holding fewer
/// than it names, is refused whole.
pub fn parse_cpc(o: &Object) -> Option<Cpc> {
    let Object::Package(p) = o else { return None };
    let e = p.borrow();
    let int = |i: usize| match e.get(i) {
        Some(Object::Integer(v)) => usize::try_from(*v).ok(),
        _ => None,
    };
    let (count, revision) = (int(0)?, u8::try_from(int(1)?).ok()?);
    if count < FIELDS - 1 || e.len() < count {
        return None;
    }
    let mut cpc = Cpc { revision, ..Cpc::default() };
    for (slot, o) in cpc.fields.iter_mut().zip(e.iter().take(count).skip(2)) {
        *slot = field(o);
    }
    Some(cpc)
}
