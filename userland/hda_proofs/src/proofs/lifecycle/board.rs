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

//! A machine for one bring-up attempt: QEMU's ICH6 HD Audio controller with
//! codecs on its link (`sim`), its register window and DMA buffers offered
//! to the shim's broker, so `setup::run` claims, resets, walks and programs
//! it exactly as it would at boot.

use std::sync::Arc;

use nonos_devmodel::{FakeBar, LiveDevice};

use crate::constants::{GCAP, STATESTS, VMAJ};
use crate::controller::bdl::RING_BYTES;
use crate::model::{live, CORB_BYTES, RIRB_BYTES, WINDOW};
use crate::sim::{self, Sim, SimCodec};

pub const DEVICE: u64 = 0x5a;
/// Four input and four output engines and 64-bit addressing.
pub const GCAP_4X4: u16 = 0x4401;
/// Bytes past the PCM ring's mapping, there to show a write that ran off it.
pub const GUARD: usize = 64;
/// Device addresses for the CORB, the RIRB, the BDL page and the PCM ring,
/// in the order an attempt maps them.
pub const LOW: [u64; 4] = [0x1000_0000, 0x1010_0000, 0x1020_0000, 0x1030_0000];

pub struct Board {
    pub sim: Sim,
    pub bdl: Arc<FakeBar>,
    pub pcm: Arc<FakeBar>,
    _codecs: LiveDevice,
}

/// A controller with `codecs` on its link. The broker hands out one DMA
/// buffer per address in `dma`, and refuses any map past them.
pub fn board(codecs: Vec<(u8, SimCodec)>, dma: &[u64]) -> Board {
    let mask = codecs.iter().fold(0u16, |m, (cad, _)| m | 1 << cad);
    let sim = sim::start(codecs);
    sim.bar.present16(GCAP as usize, GCAP_4X4);
    sim.bar.present8(VMAJ as usize, 1);
    // The codecs keep signalling, so the reset finds them however it clears.
    let codecs = live(&sim.bar, move |b| {
        b.present16(STATESTS as usize, mask);
        std::thread::yield_now();
    });
    let (bdl, ring) = (Arc::new(FakeBar::new(4096)), RING_BYTES as usize);
    let pcm = Arc::new(FakeBar::new(ring + GUARD));
    let bufs = [(&sim.corb, CORB_BYTES), (&sim.rirb, RIRB_BYTES), (&bdl, 4096), (&pcm, ring)];
    let offered = bufs.iter().zip(dma).map(|((b, n), &dev)| (b.base(), dev, *n as u64));
    nonos_libc::offer_dma(offered.collect());
    nonos_libc::offer_window(sim.bar.base(), WINDOW as u64);
    Board { sim, bdl, pcm, _codecs: codecs }
}
