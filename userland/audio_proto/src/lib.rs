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

//! The wire format of the audio service, for both ends of it.

#![no_std]

pub mod header;
pub mod ops;
pub mod output;
pub mod tone;
pub mod volume;

pub use header::{write_header, HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use ops::OP_STREAM_OPEN;
pub use ops::{E_AGAIN, E_INVAL, E_OK};
pub use ops::{OP_CLOSE, OP_FEED_PCM, OP_PAUSE, OP_PLAY_PCM, OP_PLAY_TONE, OP_RESUME, OP_STOP};
pub use output::{
    is_output_message, output_message, output_short, output_status_reply, output_status_request,
    read_output_status, E_NODEV, FLAG_HEADPHONE, FLAG_LINE_OUT, FLAG_PLUGGED, FLAG_SPEAKER,
    OP_OUTPUT_STATUS, OUTPUT_AMD_ACP, OUTPUT_HDMI_ONLY, OUTPUT_NEEDS_SOF, OUTPUT_NOT_ANSWERING,
    OUTPUT_NO_CODEC, OUTPUT_NO_DEVICE, OUTPUT_NO_PATH, OUTPUT_READY, OUTPUT_REPLY_LEN,
};
pub use tone::{tone_request, TONE_MSG_LEN, TONE_PAYLOAD_LEN};
pub use volume::{
    read_volume_reply, read_volume_request, volume_query, volume_reply, volume_request,
    MasterVolume, OP_SET_VOLUME, VOLUME_MAX, VOLUME_MSG_LEN, VOLUME_PAYLOAD_LEN, VOLUME_REPLY_LEN,
};
