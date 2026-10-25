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

//! The circuit's transitions as a straight-line program. The engine's own
//! transition code runs over the recording value, so the tape is the
//! constraints themselves, not a transcription of them.

use nonos_stark::air::{Air, GenericTransition, WiredMultiGen};
use nonos_stark::field::{Fp, Fp2};
use stark_proofs::wrap::{Op, Rec};

pub struct Recorded {
    pub ops: Vec<Op>,
    pub vals: Vec<Fp2>,
    pub outputs: Vec<u32>,
    pub n_frame: usize,
    pub n_periodic: usize,
}

/// Inputs in the image's order: the frame, the periodic values, then the
/// four challenge components. Recorded on arbitrary distinct values, since
/// the tape is a function of its inputs and not of the point it ran at.
pub fn record(air: &WiredMultiGen) -> Recorded {
    let n_frame = air.window_size() * air.trace_width();
    let n_periodic = Air::periodic_columns(air).len();
    let val = |k: usize| Fp2::new(Fp::from_u64(1000 + k as u64), Fp::from_u64(7 * k as u64 + 3));
    Rec::<Fp2>::start();
    let input = |k: usize| Rec::input(k, val(k));
    let frame: Vec<Rec<Fp2>> = (0..n_frame).map(input).collect();
    let periodic: Vec<Rec<Fp2>> = (n_frame..n_frame + n_periodic).map(input).collect();
    let chal: Vec<Rec<Fp2>> = (0..4).map(|i| input(n_frame + n_periodic + i)).collect();
    let outs = air.transition_gen_at::<Rec<Fp2>>(&frame, &periodic, &chal);
    let tape = Rec::<Fp2>::take();
    Recorded {
        ops: tape.ops,
        vals: tape.vals,
        outputs: outs.iter().map(|r| r.id()).collect(),
        n_frame,
        n_periodic,
    }
}
