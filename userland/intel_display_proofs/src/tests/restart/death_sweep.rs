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
//! One death sweep for a restart (G-78, G-79): the driver killed at each
//! step of `work`, then restarted onto exactly what it left, and judged.

use super::death_kit::{rig, steps};
use super::kit::restart;
use super::restart_tests::reaches_only_ours;
use crate::engine::pipe::Transcoder;
use crate::engine::access::Mmio;
use crate::engine::regs::trans_conf;
use crate::model::confine::names;
use crate::tests::confine::kit::{confined, FIRMWARE};
use crate::tests::deaths::kit_held::held;
use crate::tests::deaths::kit::{run, Wrong};
use crate::tests::wire_rig::Rig;

/// Restarts that took the output whole, and ones left dark.
pub fn sweep(work: fn(&mut Rig), name: &str) -> (u32, u32) {
    let (mut whole, mut dark) = (0, 0);
    for k in steps(work) {
        let (mut d, model) = rig();
        run(&model, Wrong::Killed(k), || work(&mut d));
        let kept = held(&d);
        drop(d);
        let m = std::rc::Rc::try_unwrap(model).ok().expect("the dead driver gone").into_inner();
        let reach = m.reaching();
        // A transcoder told to stop shows it a few reads later, as the restart sees it.
        let mut m = m;
        let conf = (0..4).fold(0, |_, _| Mmio::read(&mut m, trans_conf(Transcoder::B)));
        let running = conf & 1 << 30 != 0;
        let Some((_d, model)) = restart(m) else {
            // Nothing lit to take: what is reached is still the killed
            // driver's own frames, kept for good, or firmware's.
            let ok = |a: &u64| names(&kept, *a | 1) || names(&[FIRMWARE], *a | 1);
            assert!(reach.iter().all(ok), "{name} killed at {k}: dark, reaching {reach:x?}");
            assert!(!running, "{name} killed at {k}: left dark with its pipe running (G-79)");
            dark += 1;
            continue;
        };
        let (m, tag) = (model.borrow(), format!("{name} killed at {k}"));
        confined(&m, &tag);
        reaches_only_ours(&m, &tag);
        assert_eq!(m.watch.reached, 0, "{tag}: a frame given back reached");
        let px = crate::engine::pipe::Pipe::ALL.iter().find_map(|&p| m.pixel(p, 5, 5));
        assert_eq!(px, Some(0xFF00_00AA), "{tag}: the new surface shows");
        whole += 1;
    }
    (whole, dark)
}
