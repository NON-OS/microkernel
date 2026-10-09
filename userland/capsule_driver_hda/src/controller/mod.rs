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
pub(crate) mod bdl;
pub(crate) mod codec;
mod codec_probe;
mod compose;
mod immediate;
pub(crate) mod corb;
pub(crate) mod dma_sync;
mod info;
pub(crate) mod intel;
pub(crate) mod position;
pub(crate) mod reset;
pub(crate) mod sst;
mod stream_layout;
pub(crate) mod stream_run;
mod streams;
pub(crate) mod verb;
pub(crate) mod verdict;
pub(crate) mod verdict_name;
pub(crate) mod wait;

pub use codec_probe::{probe, CodecProbe, MAX_CODECS};
pub(crate) use compose::{compose_verb, compose_verb_long};
pub use info::ControllerInfo;
pub use stream_layout::layout;
pub(crate) use stream_layout::{StreamDescriptor, STREAM_BIDI, STREAM_OUTPUT};
pub(crate) use stream_run::StreamRun;
