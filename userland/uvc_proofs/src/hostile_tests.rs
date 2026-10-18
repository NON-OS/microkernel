// NONOS Operating System (AGPL-3.0-or-later)
//! A webcam's descriptors made hostile: a frame of the other kind, index
//! 0, and endpoints outside USB's bounds.

use nonos_uvc::camera;

use crate::webcam::config;

fn at(c: &[u8], pat: &[u8]) -> usize {
    c.windows(pat.len()).position(|w| w == pat).expect("in the webcam")
}

const MJPEG_FORMAT: [u8; 4] = [11, 0x24, 0x06, 2];

#[test]
fn a_frame_of_the_other_kind_joins_no_format() {
    let mut c = config();
    let mjpeg = at(&c, &MJPEG_FORMAT);
    let mut stray = c[mjpeg + 11..mjpeg + 11 + 30].to_vec();
    stray[3] = 3;
    c.splice(mjpeg..mjpeg, stray);
    let cam = camera(&c).unwrap();
    assert_eq!(cam.formats[0].frames.len(), 2, "an MJPEG frame under YUY2");
}

#[test]
fn index_0_names_nothing_and_its_frames_join_nothing() {
    let mut c = config();
    let mjpeg = at(&c, &MJPEG_FORMAT);
    c[mjpeg + 3] = 0;
    let cam = camera(&c).unwrap();
    assert_eq!(cam.formats.len(), 1);
    assert_eq!(cam.formats[0].frames.len(), 2);
    // A second YUY2 format of index 0, with a frame: kind alone would take
    // that frame into the first YUY2 format.
    let mut c = config();
    let yuy2 = at(&c, &[27, 0x24, 0x04, 1]);
    let mut refused = c[yuy2..yuy2 + 57].to_vec();
    (refused[3], refused[27 + 3]) = (0, 3);
    let mjpeg = at(&c, &MJPEG_FORMAT);
    c.splice(mjpeg..mjpeg, refused);
    assert_eq!(camera(&c).unwrap().formats[0].frames.len(), 2);
    let mut c = config();
    let second = at(&c, &[30, 0x24, 0x05, 2]);
    c[second + 3] = 0;
    assert_eq!(camera(&c).unwrap().formats[0].frames.len(), 1);
}

#[test]
fn an_endpoint_outside_usbs_bounds_carries_no_stream() {
    let alt2 = |mps: u16| {
        let mut c = config();
        let n = c.len();
        c[n - 3..n - 1].copy_from_slice(&mps.to_le_bytes());
        camera(&c).unwrap().alts.iter().map(|a| a.setting).collect::<Vec<u8>>()
    };
    assert_eq!(alt2(0x1400), vec![1, 2]);
    assert_eq!(alt2(0x1800 | 1024), vec![1], "3 added transactions is reserved");
    assert_eq!(alt2(0x2000 | 1024), vec![1], "bit 13 is reserved");
    assert_eq!(alt2(1025), vec![1]);
}
