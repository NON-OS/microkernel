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

//! The controller's state: the codec's registers, the link, the PCM out
//! box, and what the engine has played.

#[derive(Clone, Copy)]
pub struct Options {
    /// A codec on the link: its ready bit rises after a cold reset.
    pub codec: bool,
    /// The codec's DAC, mixer and Vref report ready.
    pub powered: bool,
    /// The windows are not decoded: reads float to all ones.
    pub decode_off: bool,
    /// The box reset never completes.
    pub rr_sticks: bool,
    /// Firmware left the engine running.
    pub running: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { codec: true, powered: true, decode_off: false, rr_sticks: false, running: false }
    }
}

#[derive(Default)]
pub struct Ich {
    pub opt: Options,
    /// The codec's registers, by register number over two.
    pub mixer: Vec<u16>,
    pub cas: bool,
    pub glob_cnt: u32,
    pub cold_low: bool,
    pub cold_cycled: bool,
    pub bdbar: u32,
    pub civ: u8,
    pub lvi: u8,
    pub piv: u8,
    pub sr: u16,
    pub cr: u8,
    pub picb: u16,
    pub cur: u32,
    /// The bytes played while the outputs were open, and how many while muted.
    pub heard: Vec<u8>,
    pub muted_bytes: usize,
    /// Times the engine reached the last valid entry and halted.
    pub halts: usize,
    /// Every write to either window, in order: (grant, offset, value).
    pub writes: Vec<(u64, u16, u32)>,
}

impl Ich {
    pub fn new(opt: Options) -> Self {
        // Halted, as out of a reset, unless firmware left the engine running.
        let (sr, cr) = if opt.running { (0, 1) } else { (1, 0) };
        let mut ich = Self { opt, mixer: vec![0; 64], sr, cr, ..Self::default() };
        ich.reset_codec();
        ich
    }
}
