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

mod big5;
mod css;
mod encoding;
mod euc_jp;
mod euc_kr;
mod gb18030;
mod gb18030_ranges;
mod guess;
mod index;
mod iso2022jp;
mod iso2022jp_state;
mod label;
mod meta_content;
mod prescan;
mod prescan_attr;
mod prescan_meta;
mod prescan_value;
mod shift_jis;
mod single_byte;
mod sniff;
mod utf16;
mod xml_decl;

pub use encoding::{Encoding, WINDOWS_1252};
pub use label::encoding;
pub use sniff::{decode, Doc};
