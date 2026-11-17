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

//! The calculator's readout and conversion result size by the toolkit's
//! `fit_size_by`, else the ladder's smallest, as its private `fit` did, at
//! 1x, 1.25x, 1.5x and 2x over each ladder scaled by `scale().font`. The
//! ladders are the painters' own logical sizes, copied since the painters are
//! not on the host. Its own binary, so setting the scale races no other test.

use apps_proofs::calc::error_kind::ErrorKind as E;
use apps_proofs::calc::fixed::{Fixed, FRAC};
use apps_proofs::calc::format::{error_text, format, DISPLAY_MAX};
use apps_proofs::calc::prog::{write, BASES};
use apps_proofs::calc::ui::metrics::PX_RESULT;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::paint::measure_ttf;
use nonos_toolkit::widgets::text::fit_size_by;

const READOUT: [f32; 5] = [PX_RESULT, 44.0, 34.0, 26.0, 17.0];
const RESULT: [f32; 3] = [34.0, 26.0, 17.0];
const SMALL: [Fixed; 5] = [0, 7 * FRAC, -42 * FRAC, FRAC / 3, -1];
const LARGE: [Fixed; 4] = [123_456_789_012_345_678, i128::MAX, i128::MIN, 1 << 100];

fn old_fit(fb: &PaintBuffer, text: &str, budget: i32, ladder: &[f32]) -> f32 {
    for px in ladder.iter() {
        if fb.measure_ttf(text, *px) <= budget {
            return *px;
        }
    }
    ladder[ladder.len() - 1]
}

fn samples() -> Vec<String> {
    let kinds = [E::DivByZero, E::Overflow, E::DomainError, E::None];
    let mut out: Vec<String> = kinds.iter().map(|k| error_text(*k).into()).collect();
    let mut buf = [0u8; DISPLAY_MAX];
    for (i, v) in SMALL.iter().chain(&LARGE).enumerate() {
        let n = format(*v, (i % 9) as u8, &mut buf);
        out.push(core::str::from_utf8(&buf[..n]).unwrap().into());
    }
    for v in [0i64, 255, -1, i64::MAX, i64::MIN] {
        for base in BASES {
            let n = write(v, base, &mut buf);
            out.push(core::str::from_utf8(&buf[..n]).unwrap().into());
        }
    }
    out
}

#[test]
fn every_readout_takes_the_size_the_old_fit_chose_at_every_scale() {
    let fb = PaintBuffer { pixels: &mut [0u32; 4], stride_words: 2, width: 2, height: 2 };
    let samples = samples();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (readout, result) = (READOUT.map(|p| scale().font(p)), RESULT.map(|p| scale().font(p)));
        for s in &samples {
            for ladder in [&readout[..], &result[..]] {
                let full = fb.measure_ttf(s, ladder[0]);
                for budget in -30..=full + 4 {
                    let new = fit_size_by(s, budget, ladder, measure_ttf)
                        .unwrap_or(ladder[ladder.len() - 1]);
                    assert_eq!(new, old_fit(&fb, s, budget, ladder), "{s:?} in {budget} at {q}");
                }
            }
        }
    }
    set_scale(4);
}
