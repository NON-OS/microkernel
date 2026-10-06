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
use alloc::boxed::Box;

use nonos_libc::DmaMapOut;

use crate::controller::codec::plan::Plan;
use crate::controller::codec::widget::Codec;
use crate::controller::verb::Link;
use crate::controller::{CodecProbe, MAX_CODECS};
use crate::handles::BrokerHandles;
use crate::protocol::OutputStatus;
use crate::regs::Regs;

pub struct Driver {
    pub handles: BrokerHandles,
    pub regs: Regs,
    pub link: Link,
    /// STATESTS as the reset left it; the register is cleared after reading.
    pub codec_mask: u16,
    pub codecs: [CodecProbe; MAX_CODECS],
    pub codec: Box<Codec>,
    pub plan: Plan,
    pub bdl: DmaMapOut,
    pub sample: DmaMapOut,
    pub stream_off: u32,
    pub stream_tag: u8,
    pub stream_gi: u16,
    pub stream_kind: u8,
    /// The DMA position buffer entry for this stream, when it is in use.
    pub posbuf_va: Option<u64>,
    pub status: OutputStatus,
}

/// What a bring-up ended in: a controller playing, or the plain reason none
/// can, with every claim already given back.
pub enum Started {
    Playing(Driver),
    Silent(OutputStatus),
}
