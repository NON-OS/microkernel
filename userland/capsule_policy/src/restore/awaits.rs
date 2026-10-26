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

//! Which reads wait for the restore.
//!
//! Until the restore has finished, the store holds defaults for every field
//! an earlier boot may have kept. Handed out, a default would pass for the
//! kept value: a clock in the wrong zone, a keyboard in the wrong layout, a
//! machine that seems to keep nothing. So those fields answer "not ready"
//! until then, and a client asks again or waits, as it does for no answer.
//!
//! Only `nonos_policy_proto` is named here, so the host proofs mount this
//! file as it is.

use nonos_policy_proto::settings_record::KEPT;
use nonos_policy_proto::Field;

/// True when a read of `field` must wait: the restore has not `settled`,
/// and the field is one it puts back (`Persistent` and the kept fields).
pub fn awaits(field: Field, settled: bool) -> bool {
    !settled && (field == Field::Persistent || KEPT.contains(&field))
}
