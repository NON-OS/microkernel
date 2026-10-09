// A sharp-edged baseline JPEG must decode to within a few levels of a reference decoder.
// Reading the quantisation table in the wrong order rings at every edge.

const JPEG: &[u8] = include_bytes!("fixtures/edge16.jpg");
// The same file decoded by Pillow (libjpeg), RGB rows.
const REFERENCE: &[u8] = include_bytes!("fixtures/edge16.rgb");

#[test]
fn edge_matches_reference() {
    let mut px = [0u32; 256];
    let size = nonos_toolkit::image::jpeg::decode_jpeg_argb8888(JPEG, &mut px);
    assert!(size.is_ok());
    let mut worst = 0i32;
    for (p, want) in px.iter().zip(REFERENCE.chunks(3)) {
        let got = [(p >> 16) as u8, (p >> 8) as u8, *p as u8];
        for c in 0..3 {
            worst = worst.max((got[c] as i32 - want[c] as i32).abs());
        }
    }
    assert!(worst <= 4, "worst channel error {worst}");
}
