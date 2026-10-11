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

//! EDID timings made whole, one way for every driver.

use crate::cvt::{cvt, Blanking};
use crate::resolve::{compute, resolve};
use nonos_edid::{Source, Timing};

fn named(w: u32, h: u32, hz: u32, source: Source) -> Timing {
    Timing::nominal(w, h, hz, source)
}

#[test]
fn established_and_standard_timings_are_dmt_modes() {
    let t = resolve(&named(1024, 768, 60, Source::Established)).unwrap();
    assert_eq!((t.pixel_khz, t.h_total(), t.v_total()), (65_000, 1344, 806));
    let t = resolve(&named(1920, 1080, 60, Source::Standard)).unwrap();
    assert_eq!(t.pixel_khz, 148_500);
}

#[test]
fn a_size_the_dmt_lacks_is_computed_by_cvt() {
    let t = resolve(&named(2560, 1440, 60, Source::Standard)).unwrap();
    assert_eq!(Some(t), cvt(2560, 1440, 60_000, Blanking::Standard));
    assert_eq!(
        compute(2560, 1440, 59_940, Blanking::Reduced),
        cvt(2560, 1440, 59_940, Blanking::Reduced)
    );
}

#[test]
fn a_cvt_code_with_reduced_blanking_gets_it() {
    let mut t = named(1920, 1080, 60, Source::Cvt);
    t.reduced_blanking = true;
    assert_eq!(resolve(&t).unwrap().pixel_khz, 138_500);
}

#[test]
fn detail_passes_through_and_the_rest_has_none() {
    let d = cvt(1600, 900, 60_000, Blanking::ReducedV2).unwrap();
    assert_eq!(resolve(&Timing::detailed(d, Source::Detailed)), Some(d));
    let mut i = named(1024, 768, 87, Source::Established);
    i.interlaced = true;
    assert_eq!(resolve(&i), None);
    assert_eq!(resolve(&named(1920, 1080, 60, Source::CtaVic)), None);
}
