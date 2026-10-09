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
//! Which codec on a controller the driver plays through.
//!
//! Every codec that answered is walked, not only the first: the analog codec
//! sits at address 0 on most boards, but nothing requires it, and on a
//! machine with an HDMI codec at 0 the old driver configured the display
//! codec and the speakers stayed silent. A codec with speakers is preferred
//! to one with only jacks, and a codec that fails its walk is skipped rather
//! than failing the controller.

use alloc::boxed::Box;

use super::mark::Line;
use crate::controller::codec::pincfg::OutKind;
use crate::controller::codec::plan::{plan, Class, Plan};
use crate::controller::codec::walk::walk;
use crate::controller::codec::widget::Codec;
use crate::controller::verb::Link;
use crate::controller::verdict::Findings;
use crate::controller::CodecProbe;

pub struct Choice {
    pub best: Option<(Box<Codec>, Plan)>,
    pub findings: Findings,
}

pub fn choose(link: &mut Link, codecs: &[CodecProbe], codec_mask: u16) -> Choice {
    let mut best: Option<(Box<Codec>, Plan)> = None;
    let (mut digital, mut other) = (false, false);
    for p in codecs.iter().filter(|p| p.present != 0 && p.ok != 0) {
        let codec = match walk(link, p.address) {
            Ok(Some(c)) => c,
            Ok(None) => continue,
            Err(_) => {
                Line::new("[HDA] codec ").dec(p.address as u32).s(" walk failed, skipped").emit();
                continue;
            }
        };
        match plan(&codec) {
            Class::Analog(pl) => {
                let better = match &best {
                    None => true,
                    Some((_, b)) => pl.has(OutKind::Speaker) && !b.has(OutKind::Speaker),
                };
                if better {
                    best = Some((codec, pl));
                }
            }
            Class::DigitalOnly => digital = true,
            Class::NoOutput => other = true,
        }
    }
    let findings = Findings {
        codec_mask,
        analog: best.is_some(),
        digital_only: digital && !other,
    };
    Choice { best, findings }
}
