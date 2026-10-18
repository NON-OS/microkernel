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
//! Which RIRB entry answers a verb: never an unsolicited response, and never
//! one from a codec other than the one asked.

use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::sim;

#[test]
fn unsolicited_responses_in_the_ring_are_set_aside_not_taken_as_answers() {
    /*
     * The RIRB carries unsolicited responses (a jack event) between the
     * answers. Reading each answer from the slot numbered like its command
     * took the first unsolicited response as an answer and every answer after
     * it from the wrong slot.
     */
    let mut c = sim_codec(&alc236_hp());
    c.unsol_before_each = 1;
    let s = sim::start(vec![(0, c)]);
    let mut link = s.link();
    for _ in 0..3 {
        assert_eq!(
            link.send(0x000f_0000),
            Ok(0x10ec_0236),
            "an unsolicited entry was read as the answer"
        );
    }
    assert_eq!(link.unsolicited(), 3);
}

#[test]
fn an_answer_is_taken_only_from_the_codec_that_was_asked() {
    let s = sim::start(vec![(0, sim_codec(&alc236_hp()))]);
    let mut link = s.link();
    // Codec 2 is not on this link: no answer may be borrowed from codec 0.
    assert!(link.send(0x200f_0000).is_err());
    assert_eq!(link.send(0x000f_0000), Ok(0x10ec_0236));
}
