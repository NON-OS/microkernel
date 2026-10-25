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

mod dispatch;
mod message1;
mod stepoutput;
use super::state::{State, Supplicant};
use crate::eapol::parse::{
    parse, EapolKey, DESCRIPTOR_RSN, KEY_INFO_ACK, KEY_INFO_ERROR, KEY_INFO_INDEX_MASK,
    KEY_INFO_MIC, KEY_INFO_PAIRWISE, KEY_INFO_REQUEST, KEY_INFO_VERSION_MASK,
};
use alloc::vec::Vec;
pub use stepoutput::*;
