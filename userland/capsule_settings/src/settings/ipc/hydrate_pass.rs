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

//! The one pass that reads every stored value when the panel opens, kept free
//! of IPC so its bound and its report can be proven on the host.

use super::error::IpcError;

/// Read each of `fields` with `get` and hand every value read to `store`, in
/// order, returning the first failure. A read that got no reply ends the pass:
/// each one waits out the full reply timeout, so a policy service that has
/// stopped answering would otherwise hold the first paint for one timeout per
/// field. A field the service answered with an error does not stop the rest.
pub fn hydrate_fields<F: Copy, V>(
    fields: &[F],
    mut get: impl FnMut(F) -> Result<V, IpcError>,
    mut store: impl FnMut(F, V),
) -> Option<IpcError> {
    let mut first = None;
    for &field in fields {
        match get(field) {
            Ok(value) => store(field, value),
            Err(err) => {
                first.get_or_insert(err);
                if err == IpcError::RecvTimeout {
                    break;
                }
            }
        }
    }
    first
}

/// What the status strip says after a pass that did not read every value:
/// the values on screen are then defaults, not what the policy service holds.
pub fn hydrate_report(err: IpcError) -> &'static [u8] {
    match err {
        IpcError::RecvTimeout => b"Policy service did not answer - values shown are not stored",
        _ => b"Some stored values could not be read - those shown are defaults",
    }
}

/// How long after a pass the policy store did not answer the next one runs.
/// Early in a boot the store is busy restoring settings from the disk; a
/// window opened then showed defaults for as long as it stayed open.
pub const HYDRATE_RETRY_MS: i64 = 3_000;

/// Whether a pass that ended with `err` is final. Only silence is tried again:
/// a field the store answered with an error will be answered the same way.
pub fn hydrate_final(err: Option<IpcError>) -> bool {
    err != Some(IpcError::RecvTimeout)
}
