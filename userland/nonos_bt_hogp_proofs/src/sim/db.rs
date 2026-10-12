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

//! An attribute database as a server holds it (Vol 3, Part F, 3.2): handle,
//! type as a UUID field on the wire (2 or 16 octets) and value. Built from
//! GATT declarations (Vol 3, Part G, 3.1 to 3.3) so tests write devices as
//! services and characteristics.

pub struct Attr {
    pub handle: u16,
    pub kind: Vec<u8>,
    pub value: Vec<u8>,
}

#[derive(Default)]
pub struct Db {
    pub attrs: Vec<Attr>,
}

impl Db {
    fn next(&self) -> u16 {
        self.attrs.last().map_or(1, |a| a.handle + 1)
    }

    pub fn raw(mut self, kind: &[u8], value: &[u8]) -> Self {
        let handle = self.next();
        self.attrs.push(Attr { handle, kind: kind.to_vec(), value: value.to_vec() });
        self
    }

    pub fn service(self, uuid: u16) -> Self {
        self.raw(&0x2800u16.to_le_bytes(), &uuid.to_le_bytes())
    }

    /// A characteristic declaration and its value, the UUID given as its
    /// wire field.
    pub fn char_field(self, props: u8, uuid: &[u8], value: &[u8]) -> Self {
        let decl = [&[props][..], &(self.next() + 1).to_le_bytes(), uuid].concat();
        self.raw(&0x2803u16.to_le_bytes(), &decl).raw(uuid, value)
    }

    pub fn char(self, props: u8, uuid: u16, value: &[u8]) -> Self {
        self.char_field(props, &uuid.to_le_bytes(), value)
    }

    pub fn desc(self, uuid: u16, value: &[u8]) -> Self {
        self.raw(&uuid.to_le_bytes(), value)
    }

    pub fn handle_of(&self, uuid: u16) -> u16 {
        let field = uuid.to_le_bytes();
        self.attrs.iter().find(|a| a.kind == field).expect("no such attribute").handle
    }
}
