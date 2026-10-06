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

//! Refusals while the response comes out, and the names they carry.

use super::fail::FifoFail;
use super::harness::{frame, refused};
use super::model::Model;
use crate::security::tpm::error::TpmError;

#[test]
fn a_response_larger_than_the_buffer_is_refused() {
    let mut model = Model::new(frame(40, 30));
    assert_eq!(refused(&mut model, &frame(12, 2), 32), FifoFail::ResponseSize);
}

#[test]
fn a_response_shorter_than_its_header_is_refused() {
    let mut model = Model::new(frame(6, 0));
    assert_eq!(refused(&mut model, &frame(12, 2), 32), FifoFail::ResponseSize);
}

#[test]
fn bytes_past_the_stated_size_are_refused() {
    let mut model = Model::new(frame(12, 6));
    assert_eq!(refused(&mut model, &frame(12, 2), 32), FifoFail::ResponseTrailing);
}

#[test]
fn a_response_cut_short_stalls_rather_than_hangs() {
    let mut model = Model::new(frame(20, 4));
    assert_eq!(refused(&mut model, &frame(12, 2), 32), FifoFail::ResponseStalled);
}

#[test]
fn every_refusal_has_its_own_name_and_a_transport_error() {
    use FifoFail::*;
    let all = [
        CommandTooShort,
        LocalityNotGranted,
        NotReady,
        NoBurst,
        StatusNotValid,
        ExpectDropped,
        ExpectStillSet,
        NoResponse,
        ResponseStalled,
        ResponseSize,
        ResponseTrailing,
    ];
    for (i, a) in all.iter().enumerate() {
        assert!(all[i + 1..].iter().all(|b| a.as_str() != b.as_str()), "{a:?}");
        assert!(a.as_str().is_ascii());
    }
    assert_eq!(NoResponse.error(), TpmError::Timeout);
    assert_eq!(ExpectDropped.error(), TpmError::InvalidResponse);
}
