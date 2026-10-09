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
 * The framebuffer the kernel maps from the loader's handoff, or the reason
 * it maps none. Included by path, so the checks held here are the kernel's.
 */

#[allow(dead_code)]
#[path = "../../../src/kernel_core/init/framebuffer/frame.rs"]
mod frame;

#[cfg(test)]
mod tests {
    use super::frame::{frame, Frame, Refusal};

    /* A 1366x768 panel whose GPU pads each row to 1376 pixels. */
    #[test]
    fn a_padded_pitch_maps_the_pitch_not_the_width() {
        let f = frame(0xC000_0000, 64 << 20, 1366, 768, 1376 * 4).unwrap();
        assert_eq!(f, Frame { base: 0xC000_0000, offset: 0, map_len: 1376 * 4 * 768 });
    }

    /* Firmware that reports the whole aperture as FrameBufferSize: only the
     * frame is mapped, never the 256 MiB. */
    #[test]
    fn an_aperture_size_maps_only_the_frame() {
        let f = frame(0x40_0000_0000, 256 << 20, 3840, 2160, 3840 * 4).unwrap();
        assert_eq!(f.map_len, 3840 * 4 * 2160);
    }

    #[test]
    fn a_frame_off_a_page_boundary_maps_from_its_page() {
        let f = frame(0xE000_0400, 8 << 20, 1024, 768, 4096).unwrap();
        assert_eq!((f.base, f.offset, f.map_len), (0xE000_0000, 0x400, 0x400 + 4096 * 768));
    }

    #[test]
    fn each_refusal_is_named() {
        assert_eq!(frame(0, 1 << 20, 640, 480, 2560), Err(Refusal::Empty));
        assert_eq!(frame(0x1000, 1 << 20, 0, 480, 2560), Err(Refusal::Empty));
        /* A pixel pitch handed over as bytes would be 1920 here. */
        assert_eq!(frame(0x1000, 16 << 20, 1920, 1080, 1920), Err(Refusal::PitchBelowRow));
        assert_eq!(frame(0x1000, 1 << 20, 1920, 1080, 7680), Err(Refusal::SizeBelowFrame));
        for r in
            [Refusal::Empty, Refusal::PitchBelowRow, Refusal::Overflow, Refusal::SizeBelowFrame]
        {
            assert!(!r.says().is_empty());
        }
    }

    #[test]
    fn exactly_the_frame_is_enough() {
        assert!(frame(0x1000, 1920 * 4 * 1080, 1920, 1080, 7680).is_ok());
        assert_eq!(
            frame(0x1000, 1920 * 4 * 1080 - 1, 1920, 1080, 7680),
            Err(Refusal::SizeBelowFrame)
        );
    }
}
