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

/* WebP: the RIFF container (simple and extended), lossless VP8L (bit
 * reader, canonical Huffman, meta-Huffman groups, LZ77 with a colour
 * cache, the four inverse transforms) and lossy VP8 key frames with an
 * optional ALPH plane. */

mod bitread;
mod code_len_code;
mod color_cache;
mod color_tf;
mod container;
mod decode_image;
mod decode_pixels;
mod dist;
mod green_tf;
mod header;
mod hgroup;
mod huffman;
mod indexing;
mod lz77;
mod meta;
mod predict_fns;
mod predictor;
mod read_code_lengths;
mod read_huffman;
mod read_transform;
mod transform;
mod vp8;
mod vp8l;

pub(super) use container::decode_webp;
