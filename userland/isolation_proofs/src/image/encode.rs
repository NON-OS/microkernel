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

//! program.bin as nox_verify's Program::parse reads it, big-endian:
//! the counts, log t, the exempt rows, the ops, the outputs, the boundary
//! rows as g^row, and the boundaries, each a constant or a public word.

use super::ops::{fp, ops, u16s};
use super::record::Recorded;
use nonos_stark::air::{Air, WiredMultiGen};
use nonos_stark::fri::root_of_unity;

pub fn encode(air: &WiredMultiGen, r: &Recorded) -> Vec<u8> {
    let bnd = air.boundary();
    let public = air.wired().public_pin_range();
    let mut rows: Vec<usize> = bnd.iter().map(|b| b.1).collect();
    rows.sort_unstable();
    rows.dedup();
    let log_t = air.log_trace_len();
    let (t, g) = (1usize << log_t, root_of_unity(log_t));
    let exempt = air.window_size() - 1;
    let mut out = Vec::new();
    for n in [r.ops.len(), r.outputs.len(), bnd.len(), rows.len(), r.n_frame, r.n_periodic] {
        u16s(&mut out, n);
    }
    out.push(log_t as u8);
    out.push(exempt as u8);
    (0..exempt).for_each(|k| fp(&mut out, g.pow((t - 1 - k) as u64)));
    ops(&mut out, r);
    r.outputs.iter().for_each(|&o| u16s(&mut out, o as usize));
    rows.iter().for_each(|&row| fp(&mut out, g.pow(row as u64)));
    for (i, &(col, row, v)) in bnd.iter().enumerate() {
        out.push(u8::try_from(col).expect("a column past u8"));
        u16s(&mut out, rows.binary_search(&row).expect("a listed row"));
        if public.contains(&i) {
            out.extend_from_slice(&[1, (i - public.start) as u8]);
        } else {
            out.push(0);
            fp(&mut out, v);
        }
    }
    out
}
