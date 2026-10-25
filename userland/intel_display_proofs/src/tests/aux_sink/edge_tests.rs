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

//! E-DDC over AUX one case at a time: every block of an honest sink, a
//! pointer refused or ignored, a block cut, shifted or repeated.

use super::regs::AuxRegs;
use super::sink::{AuxSink, Eddc, Fault};
use super::sweep_tests::DP_B;
use crate::display::{read_edid, EDID_MAX};
use crate::model::Ticks;
use crate::tests::ddc_sink::kit::honest;

pub fn read(s: AuxSink) -> (usize, Vec<u8>, AuxSink) {
    let t = Ticks::new();
    let mut regs = AuxRegs::new(&t, s);
    let mut out = [0xA5u8; EDID_MAX];
    let n = read_edid(&mut regs, &t, &DP_B, &mut out);
    regs.sink.close(t.now.get());
    (n, out.to_vec(), regs.sink)
}

#[test]
fn an_honest_dp_sink_gives_every_block_through_the_pointer() {
    let (n, out, s) = read(AuxSink::new(honest(4)));
    assert_eq!((n, &out[..]), (512, &honest(4)[..]));
    assert_eq!(s.log[2].segment, Some((1, true)));
    assert!(s.log.iter().all(|t| t.stop_ok), "every transaction stopped");
}

#[test]
fn a_dp_sink_refusing_or_ignoring_the_pointer_gives_its_first_segment() {
    for eddc in [Eddc::Nak, Eddc::Ignore] {
        let mut s = AuxSink::new(honest(4));
        s.eddc = eddc;
        let (n, out, s) = read(s);
        assert_eq!((n, &out[..256]), (256, &honest(4)[..256]), "{eddc:?}");
        assert_eq!(s.log.len(), 5, "{eddc:?}: two blocks, then three tries at block 2");
    }
}

#[test]
fn a_dp_block_cut_is_read_again_and_one_shifted_or_repeated_never_lands() {
    let mut s = AuxSink::new(honest(1));
    s.faults = vec![Fault::Nak(16), Fault::Stall(9)];
    let (n, out, _) = read(s);
    assert_eq!((n, &out[..128]), (128, &honest(1)[..]), "cut twice, then whole");
    let mut s = AuxSink::new(honest(2));
    s.faults = vec![Fault::None, Fault::Drop(40), Fault::Again(3), Fault::Index(1)];
    let (n, out, _) = read(s);
    assert!(n == 128 && out[128..].iter().all(|&b| b == 0xA5), "shifted three times");
    let mut s = AuxSink::new(honest(2));
    s.ignore_index = true;
    assert_eq!(read(s).0, 128, "block 1 repeating the base is refused");
}
