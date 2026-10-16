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

use super::out_pins::{pins, unrouted};
use super::path::{find, Miss, Path, MAX_DEPTH};
use super::pincfg::OutKind;
use super::plan::{Class, Output, Plan, MAX_OUTPUTS};
use super::widget::Codec;
use crate::constants::PINCAP_PRES_DETECT;

const EMPTY: Path = Path { nodes: [0; MAX_DEPTH], sel: [0; MAX_DEPTH], len: 0 };

pub fn plan(codec: &Codec) -> Class {
    let mut plan = Plan { outputs: [None; MAX_OUTPUTS] };
    let mut paths = [EMPTY; MAX_OUTPUTS];
    let (mut n, mut spent) = (0usize, false);
    for kind in [OutKind::Speaker, OutKind::Headphone, OutKind::LineOut] {
        let (pins, k) = pins(codec, kind);
        for &(_, pin) in &pins[..k] {
            if n >= MAX_OUTPUTS {
                break;
            }
            let path = match find(codec, pin, &paths[..n]) {
                Ok(path) => path,
                Err(miss) => {
                    spent |= miss == Miss::OverBudget;
                    continue;
                }
            };
            let w = codec.get(pin).map(|w| (w.pin_caps, w.pin_cfg));
            let senses = w.is_some_and(|(caps, cfg)| {
                caps & PINCAP_PRES_DETECT != 0 && cfg.is_jack() && !cfg.presence_overridden()
            });
            paths[n] = path;
            plan.outputs[n] = Some(Output { kind, path, senses });
            n += 1;
        }
    }
    match (n, spent) {
        (0, true) => Class::OverBudget,
        (0, false) => unrouted(codec),
        _ => Class::Analog(plan),
    }
}
