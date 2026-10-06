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

//! A certificate that chains to no root is never answered.

use super::answer_rfc8448::request;
use super::rfc8448_flight::{coalesced, keyed};
use crate::handshake_state::{Progress, Refusal};

#[test]
fn a_chain_to_no_root_is_refused_before_anything_is_sealed() {
    let flight = coalesced();
    let mut state = keyed(&flight);
    assert_eq!(state.advance(&flight), Progress::Complete(flight.len()));
    let refused = state.answer(b"server", 20260101000000, &request()).err();
    assert_eq!(refused, Some(Refusal::Unverified));
}
