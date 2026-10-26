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
//! The Cirrus Logic setup Linux applies to every CS4206, CS4207 and CS4208,
//! whatever machine it is in (sound/hda/codecs/cirrus/cs420x.c at
//! 21a7f44fd076, read for its values alone, no code taken):
//!
//! - CS4206 and CS4207: `cs_errata_init_verbs` (cs420x.c:214-235), Cirrus
//!   errata ER880C3 item 6, which runs the S/PDIF converters once so their
//!   RAMs stop leaking supply current; then `cs_coef_init_verbs` (167-184):
//!   the DACs soft-ramped, muted on a FIFO error, high-pass filtered and
//!   without coefficient auto-increment, the ADCs soft-ramped, and beep
//!   through DAC1 to DAC3. All behind the vendor widget 0x11.
//! - CS4208: `cs4208_coef_init_verbs` (186-193), behind widget 0x24.
//!
//! The Apple GPIOs are `vendor_gpio`; per-machine pin overrides are not
//! here. Unverified on hardware.

use super::widget::Codec;
use crate::controller::verb::Link;
use crate::controller::{compose_verb, compose_verb_long};
use crate::error::HdaResult;

pub use super::cirrus_verbs::{Verb, CS4208, CS420X};

/// The verbs codec `vendor_id` needs, none for a codec not Cirrus's.
pub const fn verbs(vendor_id: u32) -> &'static [Verb] {
    match vendor_id {
        0x1013_4206 | 0x1013_4207 => &CS420X,
        0x1013_4208 => &CS4208,
        _ => &[],
    }
}

pub fn init(link: &mut Link, codec: &Codec) -> HdaResult<()> {
    for &(nid, verb, v) in verbs(codec.vendor_id) {
        let cmd = if verb <= 0xf {
            compose_verb_long(codec.cad, nid, verb, v)
        } else {
            compose_verb(codec.cad, nid, verb, v)
        };
        link.send(cmd)?;
    }
    Ok(())
}
