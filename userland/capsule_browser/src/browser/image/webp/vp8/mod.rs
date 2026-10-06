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

/* Lossy VP8 key frames (RFC 6386): boolean entropy decoding, intra
 * prediction, residual tokens, inverse transforms, the loop filter, and
 * YUV 4:2:0 out to ARGB rows. */

mod alpha;
mod avg;
mod bits;
mod bits_ext;
mod bmode_tree;
mod bmodes_a;
mod bmodes_b;
mod coeffs;
mod decode;
mod fields;
mod filter;
mod filter_hdr;
mod filter_ops;
mod filter_taps;
mod header;
mod idct;
mod macroblocks;
mod modes;
mod partitions;
mod planes;
mod predict;
mod predict4;
mod predict4_ext;
mod predict4_h;
mod probs_a;
mod probs_b;
mod quant_tables;
mod recon;
mod recon_edges;
mod residuals;
mod residuals_uv;
mod tables;
mod token_probs;
mod update_a;
mod update_b;
mod yuv;
mod yuv_rgb;
mod yuv_up;

pub(super) use decode::decode_vp8;
