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
//! The controller tree from the shipping files, every module public so the
//! tests reach what they prove and nothing reads as dead for lack of the
//! capsule's own callers.

#![allow(dead_code)]

#[path = "../../../capsule_driver_hda/src/controller/bdl.rs"]
pub mod bdl;
#[path = "../../../capsule_driver_hda/src/controller/codec/mod.rs"]
pub mod codec;
#[path = "../../../capsule_driver_hda/src/controller/codec_probe.rs"]
pub mod codec_probe;
#[path = "../../../capsule_driver_hda/src/controller/compose.rs"]
pub mod compose;
#[path = "../../../capsule_driver_hda/src/controller/corb.rs"]
pub mod corb;
#[path = "../../../capsule_driver_hda/src/controller/immediate.rs"]
pub mod immediate;
#[path = "../../../capsule_driver_hda/src/controller/dma_sync.rs"]
pub mod dma_sync;
#[path = "../../../capsule_driver_hda/src/controller/info.rs"]
pub mod info;
#[path = "../../../capsule_driver_hda/src/controller/intel.rs"]
pub mod intel;
#[path = "../../../capsule_driver_hda/src/controller/position.rs"]
pub mod position;
#[path = "../../../capsule_driver_hda/src/controller/reset.rs"]
pub mod reset;
#[path = "../../../capsule_driver_hda/src/controller/sst.rs"]
pub mod sst;
#[path = "../../../capsule_driver_hda/src/controller/stream_layout.rs"]
pub mod stream_layout;
#[path = "../../../capsule_driver_hda/src/controller/stream_run.rs"]
pub mod stream_run;
#[path = "../../../capsule_driver_hda/src/controller/streams.rs"]
pub mod streams;
#[path = "../../../capsule_driver_hda/src/controller/verb.rs"]
pub mod verb;
#[path = "../../../capsule_driver_hda/src/controller/verdict.rs"]
pub mod verdict;
#[path = "../../../capsule_driver_hda/src/controller/verdict_name.rs"]
pub mod verdict_name;
#[path = "../../../capsule_driver_hda/src/controller/wait.rs"]
pub mod wait;

pub use codec_probe::{probe, CodecProbe, MAX_CODECS};
pub(crate) use compose::{compose_verb, compose_verb_long};
pub use info::ControllerInfo;
pub use stream_layout::layout;
pub use stream_layout::{StreamDescriptor, STREAM_BIDI, STREAM_OUTPUT};

