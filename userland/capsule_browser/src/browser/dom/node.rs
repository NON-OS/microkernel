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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Document,
    Element,
    Text,
}

/// The namespace an element was created in. The parser decides it: `svg`
/// and `math` open foreign content, whose elements keep their own name case
/// (`clipPath`, `foreignObject`) and follow their own nesting rules.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Ns {
    #[default]
    Html,
    Svg,
    MathMl,
}

pub struct Node {
    pub kind: NodeKind,
    pub tag: String,
    pub text: String,
    pub attrs: Vec<(String, String)>,
    pub parent: usize,
    pub children: Vec<usize>,
    pub ns: Ns,
}

impl Node {
    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.iter().find(|(k, _)| k.eq_ignore_ascii_case(key)).map(|(_, v)| v.as_str())
    }

    /// This element as a fragment context for `parse_fragment`: its name,
    /// after "svg " or "math " when it is not an HTML element.
    pub fn context_tag(&self) -> String {
        let prefix = match self.ns {
            Ns::Html => "",
            Ns::Svg => "svg ",
            Ns::MathMl => "math ",
        };
        let mut tag = String::from(prefix);
        tag.push_str(&self.tag);
        tag
    }
}
