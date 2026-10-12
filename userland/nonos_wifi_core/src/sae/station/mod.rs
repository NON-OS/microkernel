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

mod accessors;
mod commit;
mod confirm;
mod frame;
mod saestate;
mod saestation;
use super::commit::{confirm, Commit, SaeKeys};
use super::frame::{
    commit_body, confirm_body, parse_commit, parse_confirm, parse_token_request, SEQ_COMMIT,
    SEQ_CONFIRM, STATUS_ANTI_CLOGGING_TOKEN_REQ, STATUS_GROUP_NOT_SUPPORTED,
    STATUS_SAE_HASH_TO_ELEMENT, STATUS_SUCCESS, TOKEN_MAX,
};
use super::group::LEN;
use alloc::vec::Vec;
use nonos_p256::ProjectivePoint;
pub use saestate::*;
pub use saestation::*;
