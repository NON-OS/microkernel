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

//! A codec's analog output pins, by kind and in order, and what a codec is
//! when none of them could be routed.

use super::pincfg::OutKind;
use super::plan::Class;
use super::widget::Codec;
use crate::constants::{PINCAP_OUT, WIDGET_TYPE_PIN};

/// The output pins of `kind` this driver can drive, at most sixteen, sorted
/// by association and sequence.
pub(super) fn pins(codec: &Codec, kind: OutKind) -> ([(u8, u8); 16], usize) {
    let mut pins = [(0u8, 0u8); 16];
    let mut k = 0usize;
    for w in codec.all() {
        if k < pins.len()
            && w.ty() == WIDGET_TYPE_PIN
            && w.pin_caps & PINCAP_OUT != 0
            && !w.is_digital()
            && w.pin_cfg.out_kind() == Some(kind)
        {
            pins[k] = (w.pin_cfg.order(), w.nid);
            k += 1;
        }
    }
    pins[..k].sort_unstable();
    (pins, k)
}

/// A codec with no output routed. A display codec's unused ports are marked
/// not connected, so it is known by its pins: output pins, every one of them
/// HDMI or DisplayPort. An analog codec's S/PDIF pin does not make it one.
pub(super) fn unrouted(codec: &Codec) -> Class {
    let outs =
        || codec.all().iter().filter(|w| w.ty() == WIDGET_TYPE_PIN && w.pin_caps & PINCAP_OUT != 0);
    if outs().any(|w| w.is_digital()) && outs().all(|w| w.is_digital()) {
        Class::DigitalOnly
    } else {
        Class::NoOutput
    }
}
