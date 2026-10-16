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

use super::Driver;
use crate::constants::STREAM_TAG;
use crate::controller::codec::{plan::Plan, widget::Codec};
use crate::controller::verb::Link;
use crate::controller::{CodecProbe, ControllerInfo, StreamDescriptor, MAX_CODECS};
use crate::handles::BrokerHandles;
use crate::protocol::OutputStatus;
use crate::regs::Regs;

/// A controller out of reset with its verb link up, and the codecs on it:
/// what an attempt carries into `play`, and from there into the `Driver`.
pub(super) struct Linked {
    pub handles: BrokerHandles,
    pub epoch: u64,
    pub regs: Regs,
    pub info: ControllerInfo,
    pub link: Link,
    pub codec_mask: u16,
    pub codecs: [CodecProbe; MAX_CODECS],
}

/// What making it ready to play adds: the codec and its plan, the stream's
/// memory and engine, and the status the driver reports.
pub(super) struct Ready {
    pub codec: Box<Codec>,
    pub plan: Plan,
    pub bdl: DmaMapOut,
    pub sample: DmaMapOut,
    pub out: StreamDescriptor,
    pub posbuf_va: Option<u64>,
    pub status: OutputStatus,
}

impl Linked {
    /// The `Driver` this controller becomes once its stream is ready.
    pub(super) fn into_driver(self, r: Ready) -> Driver {
        Driver {
            handles: self.handles,
            regs: self.regs,
            link: self.link,
            codec_mask: self.codec_mask,
            codecs: self.codecs,
            codec: r.codec,
            plan: r.plan,
            bdl: r.bdl,
            sample: r.sample,
            stream_off: r.out.mmio_offset,
            stream_tag: STREAM_TAG,
            stream_gi: r.out.global_index,
            stream_kind: r.out.kind,
            posbuf_va: r.posbuf_va,
            status: r.status,
        }
    }
}
