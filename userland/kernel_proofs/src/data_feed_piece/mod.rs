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

/*
 * MkDataFeed's pieces: each starts where the last ended, none is longer
 * than a piece, together they cover the feed exactly, and a caller buffer
 * whose end would wrap the address space is refused before any copy.
 */

#[path = "../../../../src/syscall/microkernel/data/feed_piece.rs"]
mod feed_piece;

#[cfg(test)]
mod tests {
    use super::feed_piece::next_piece;

    const PIECE: u64 = 64 << 10;

    fn pieces(buf: u64, len: u64) -> Option<Vec<(u64, u64)>> {
        let mut out = Vec::new();
        let mut done = 0u64;
        while done < len {
            let (src, n) = next_piece(buf, len, done, PIECE)?;
            out.push((src, n));
            done += n;
        }
        Some(out)
    }

    #[test]
    fn pieces_cover_the_feed_in_order_and_no_more() {
        for len in [1u64, 100, PIECE - 1, PIECE, PIECE + 1, 3 * PIECE + 7, 1 << 20] {
            let buf = 0x4000_0000u64;
            let p = pieces(buf, len).unwrap();
            let mut at = buf;
            for &(src, n) in &p {
                assert_eq!(src, at);
                assert!(n > 0 && n <= PIECE);
                at += n;
            }
            assert_eq!(at, buf + len);
        }
    }

    #[test]
    fn a_buffer_that_wraps_is_refused() {
        assert_eq!(next_piece(u64::MAX - 1, 2, 1, PIECE), Some((u64::MAX, 1)));
        assert_eq!(next_piece(u64::MAX, 2, 2, PIECE), None);
        assert_eq!(next_piece(u64::MAX - 10, 1 << 20, PIECE, PIECE), None);
        assert!(pieces(u64::MAX - (PIECE / 2), 1 << 20).is_none());
    }

    #[test]
    fn done_past_the_length_is_no_piece() {
        assert_eq!(next_piece(0x1000, 10, 11, PIECE), None);
        assert_eq!(next_piece(0x1000, 10, 10, PIECE), Some((0x100a, 0)));
    }
}
