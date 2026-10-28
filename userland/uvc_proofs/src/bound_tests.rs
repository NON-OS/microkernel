// NONOS Operating System (AGPL-3.0-or-later)
//! A committed frame's bound, held to the frame the camera described: the
//! probe answer's dwMaxVideoFrameSize believed only up to four bytes a
//! pixel of that frame, and a format or frame never described refused.

use nonos_uvc::{camera, frame_bound, frame_cap, Camera, Probe, Refused};

use crate::webcam::config;

fn agreed(format: u8, frame: u8, max_frame: u32) -> Probe {
    Probe { format, frame, max_frame, ..Probe::default() }
}

/// The cap a capsule on the default 16 MiB heap holds frames to.
const CAP: u64 = frame_cap(16 << 20);

fn bound(c: &Camera, p: Probe) -> Result<u32, Refused> {
    frame_bound(c, &p, CAP)
}

fn cam() -> Camera {
    camera(&config()).unwrap()
}

#[test]
fn the_device_bound_is_taken_within_the_frame() {
    assert_eq!(bound(&cam(), agreed(1, 1, 614_400)), Ok(614_400));
    assert_eq!(bound(&cam(), agreed(1, 1, 0)), Ok(614_400), "the descriptor's");
    assert_eq!(bound(&cam(), agreed(1, 2, 320 * 240 * 4)), Ok(320 * 240 * 4));
}

#[test]
fn a_bound_past_four_bytes_a_pixel_is_refused() {
    assert_eq!(bound(&cam(), agreed(1, 1, 640 * 480 * 4 + 1)), Err(Refused::Undescribed));
    assert_eq!(bound(&cam(), agreed(2, 1, u32::MAX)), Err(Refused::Undescribed));
}

#[test]
fn a_format_or_frame_never_described_is_refused() {
    assert_eq!(bound(&cam(), agreed(1, 9, 1000)), Err(Refused::Undescribed));
    assert_eq!(bound(&cam(), agreed(7, 1, 1000)), Err(Refused::Undescribed));
}

#[test]
fn a_huge_frame_is_held_to_the_heap() {
    let mut c = cam();
    let fr = &mut c.formats[0].frames[0];
    (fr.width, fr.height, fr.max_bytes) = (u16::MAX, u16::MAX, 0);
    assert_eq!(bound(&c, agreed(1, 1, 0)), Err(Refused::TooLarge));
    assert_eq!(bound(&c, agreed(1, 1, CAP as u32)), Ok(CAP as u32));
    assert_eq!(bound(&c, agreed(1, 1, CAP as u32 + 1)), Err(Refused::TooLarge));
}
