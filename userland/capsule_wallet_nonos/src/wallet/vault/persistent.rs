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

//! Whether this boot keeps anything past power off.
//!
//! The person chooses at setup, and the policy service holds the choice. The
//! store asks the same question before every persist (`capsule_vfs`
//! `persist_gate`) and refuses a record on a boot that keeps nothing. It lets
//! a cleared record through, so a wallet that wrote its records here on a
//! live boot would clear the vault on the stick and then be refused the new
//! one. The keep asks first instead, and writes nothing on such a boot.

use nonos_policy_client::{get_bool, lookup};
use nonos_policy_proto::Field;

/// True when this boot keeps data across reboots. A policy service that
/// does not answer is taken as a boot that keeps nothing, as the store does.
pub fn persistent() -> bool {
    lookup().and_then(|port| get_bool(port, Field::Persistent)) == Some(true)
}
