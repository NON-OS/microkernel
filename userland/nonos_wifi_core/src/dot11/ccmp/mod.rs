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

mod aad_nonce;
mod ccmp_hdr_len;
mod encrypt;
mod view;
use super::header::{fc_subtype, fc_type, FC_FROM_DS, FC_TO_DS, MAC_HEADER_LEN, TYPE_DATA};
use crate::ccmp::ccm::ccm_decrypt;
pub use aad_nonce::*;
use alloc::vec::Vec;
pub use ccmp_hdr_len::*;
pub use encrypt::*;
pub use view::*;
