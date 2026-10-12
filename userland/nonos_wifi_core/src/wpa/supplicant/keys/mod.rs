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

mod group_keys;
mod unwrapped_max;
use super::state::Supplicant;
use crate::ccmp::keywrap::{aes_unwrap, MAX_BLOCKS};
use crate::eapol::kde::{parse_key_data, KeyData};
pub(in crate::wpa::supplicant) use group_keys::*;
pub(in crate::wpa::supplicant) use unwrapped_max::*;
