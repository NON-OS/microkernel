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

//! Which outputs of a codec this driver drives, and how.
//!
//! Every pin the board's pin configuration names as a speaker, headphone or
//! line out, and that can drive output, gets a path: speakers first, then
//! headphones, then line outs, each group in the association and sequence
//! order Linux sorts by. All of them play the one stream; switching between
//! speaker and headphone is the jack's job (`jack`). A codec with output
//! pins that are all HDMI or DisplayPort is a display codec, and one with
//! none this driver can route has no usable output.

use super::path::{find, Path, MAX_DEPTH};
use super::pincfg::OutKind;
use super::widget::Codec;
use crate::constants::{PINCAP_OUT, PINCAP_PRES_DETECT, WIDGET_TYPE_PIN};

pub const MAX_OUTPUTS: usize = 6;

const EMPTY: Path = Path { nodes: [0; MAX_DEPTH], sel: [0; MAX_DEPTH], len: 0 };

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Output {
    pub kind: OutKind,
    pub path: Path,
    /// The jack reports whether something is plugged in.
    pub senses: bool,
}

impl Output {
    pub fn pin(&self) -> u8 {
        self.path.nodes[0]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Plan {
    pub outputs: [Option<Output>; MAX_OUTPUTS],
}

impl Plan {
    pub fn iter(&self) -> impl Iterator<Item = &Output> {
        self.outputs.iter().flatten()
    }

    pub fn has(&self, kind: OutKind) -> bool {
        self.iter().any(|o| o.kind == kind)
    }

    /// The headphone jack that decides whether the speakers play.
    pub fn sensing_headphone(&self) -> Option<u8> {
        self.iter().find(|o| o.kind == OutKind::Headphone && o.senses).map(|o| o.pin())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    /// At least one analog output with a path to a DAC.
    Analog(Plan),
    /// Output pins, all of them HDMI or DisplayPort.
    DigitalOnly,
    /// Nothing this driver can play through.
    NoOutput,
}

pub fn plan(codec: &Codec) -> Class {
    let mut plan = Plan { outputs: [None; MAX_OUTPUTS] };
    let mut paths = [EMPTY; MAX_OUTPUTS];
    let mut n = 0usize;
    for kind in [OutKind::Speaker, OutKind::Headphone, OutKind::LineOut] {
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
        for &(_, pin) in &pins[..k] {
            if n >= MAX_OUTPUTS {
                break;
            }
            let Some(path) = find(codec, pin, &paths[..n]) else { continue };
            let w = codec.get(pin).map(|w| (w.pin_caps, w.pin_cfg));
            let senses = w.is_some_and(|(caps, cfg)| {
                caps & PINCAP_PRES_DETECT != 0 && cfg.is_jack() && !cfg.presence_overridden()
            });
            paths[n] = path;
            plan.outputs[n] = Some(Output { kind, path, senses });
            n += 1;
        }
    }
    if n > 0 {
        return Class::Analog(plan);
    }
    // A display codec's unused ports are marked not connected, so it is
    // known by its pins: output pins, every one of them HDMI or DisplayPort.
    // An analog codec's S/PDIF pin does not make it one.
    let outs = || codec.all().iter().filter(|w| w.ty() == WIDGET_TYPE_PIN && w.pin_caps & PINCAP_OUT != 0);
    let digital = outs().any(|w| w.is_digital()) && outs().all(|w| w.is_digital());
    if digital {
        Class::DigitalOnly
    } else {
        Class::NoOutput
    }
}
