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

use alloc::vec::Vec;

#[path = "http_get.rs"]
mod http_get;
#[path = "http_parse.rs"]
mod http_parse;
#[path = "http_put.rs"]
mod http_put;

pub use http_get::{build_get, build_get_ka, build_head};
pub use http_parse::parse_head;
pub use http_put::build_put_head;

pub struct Parsed {
    pub status: u16,
    pub body_off: usize,
    pub content_length: Option<usize>,
    pub chunked: bool,
    pub gzip: bool,
    pub location: Option<Vec<u8>>,
}
