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

//! Ending the streams of a circuit that has died.

use super::state::Manager;

/// A circuit marked Dead carries nothing more, so every stream on it is
/// ended with REASON_DESTROY. Left alone, a reader of one of them saw an
/// empty stream until its own deadline instead of a closed one it could
/// retry elsewhere.
pub(super) fn end_streams(state: &mut Manager, index: usize) {
    let circuit = state.circuits[index].id;
    for stream in state.streams.iter_mut().filter(|s| s.circuit == circuit) {
        stream.end_with_circuit();
    }
}
