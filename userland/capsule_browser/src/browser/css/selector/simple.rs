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
use alloc::vec::Vec;

use super::attr::AttrTest;
use super::comb::Comb;
use super::pseudo::Pseudo;

/* One compound selector: tag.class#id[attr=v]:pseudo. Names are stored
 * unescaped; tags and attribute names in ASCII lower case. */
#[derive(Clone)]
pub struct Simple {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attrs: Vec<(String, AttrTest)>,
    pub pseudo: Vec<Pseudo>,
    /* name_hash of each class, in the order of `classes`. */
    pub class_keys: Vec<u64>,
    /* id_key of the id, 0 without one. */
    pub id_key: u64,
}

impl Simple {
    pub fn empty() -> Self {
        Simple {
            tag: None,
            id: None,
            classes: Vec::new(),
            attrs: Vec::new(),
            pseudo: Vec::new(),
            class_keys: Vec::new(),
            id_key: 0,
        }
    }
}

/* A compound left of the key and the combinator joining it to the compound
 * on its right. */
#[derive(Clone)]
pub struct Step {
    pub simple: Simple,
    pub comb: Comb,
}
