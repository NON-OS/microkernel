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

use alloc::vec;
use alloc::vec::Vec;

use super::bits::Bools;
use super::fields::Strength;
use super::header::Header;
use super::modes::parse_mb;
use super::recon::{reconstruct, Frame};
use super::residuals::{residuals, Nz};

/// Read and reconstruct every macroblock in raster order, each row's
/// tokens from its partition, and return the loop-filter strength each
/// one takes (inner edges filter when it carried coefficients).
pub(super) fn macroblocks(
    hdr: &Header<'_>,
    br: &mut Bools<'_>,
    f: &mut Frame,
) -> Option<Vec<Strength>> {
    let (mbw, mbh) = (hdr.mbw, hdr.mbh);
    let mut tokens: Vec<Bools> = hdr.parts.iter().map(|p| Bools::new(p)).collect();
    let (mut top_modes, mut top_nz) = (vec![0u8; mbw * 4], vec![Nz::default(); mbw]);
    let mut strengths: Vec<Strength> = Vec::new();
    strengths.try_reserve_exact(mbw * mbh).ok()?;
    let mut c = [0i16; 384];
    for mby in 0..mbh {
        let (mut left_modes, mut left_nz) = ([0u8; 4], Nz::default());
        let token = &mut tokens[mby % hdr.parts.len()];
        for mbx in 0..mbw {
            let mb = parse_mb(
                br,
                hdr.seg_map,
                hdr.skip_prob,
                &mut top_modes[mbx * 4..mbx * 4 + 4],
                &mut left_modes,
            );
            let coded = if mb.skip {
                c.fill(0);
                let (t, l) = (&mut top_nz[mbx], &mut left_nz);
                let (tdc, ldc) = if mb.i4x4 { (t.dc, l.dc) } else { (false, false) };
                (*t, *l) = (Nz { dc: tdc, ..Nz::default() }, Nz { dc: ldc, ..Nz::default() });
                false
            } else {
                residuals(
                    token,
                    &hdr.probs,
                    &hdr.quant[mb.seg],
                    mb.i4x4,
                    (&mut top_nz[mbx], &mut left_nz),
                    &mut c,
                )
            };
            reconstruct(f, (mbx, mby), &mb, &c);
            let st = hdr.strength[mb.seg][mb.i4x4 as usize];
            strengths.push(Strength { inner: st.inner || coded, ..st });
        }
    }
    Some(strengths)
}
