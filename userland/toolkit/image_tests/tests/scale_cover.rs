// The wallpaper scaler: exact at 1:1, flat colours stay flat, shrinking
// averages by area, enlarging interpolates, and a different aspect crops.
use nonos_toolkit::image::scale::scale_cover;

fn run(src: &[u32], sw: u32, sh: u32, dw: u32, dh: u32) -> Vec<u32> {
    let mut out = vec![0u32; (dw * dh) as usize];
    let ok = scale_cover(src, sw, sh, dw, dh, |y, row| {
        out[(y * dw) as usize..((y + 1) * dw) as usize].copy_from_slice(row);
    });
    assert!(ok);
    out
}

fn grey(v: u32) -> u32 {
    0xFF00_0000 | (v << 16) | (v << 8) | v
}

#[test]
fn identity_is_exact() {
    let src: Vec<u32> = (0..64u32 * 36).map(|i| grey((i * 7) % 256)).collect();
    assert_eq!(run(&src, 64, 36, 64, 36), src);
}

#[test]
fn flat_stays_flat_at_every_size() {
    let src = vec![0xFF12_3456u32; 192 * 108];
    for (dw, dh) in [(128, 72), (137, 77), (384, 216), (160, 100), (100, 160)] {
        assert!(run(&src, 192, 108, dw, dh).iter().all(|&p| p == 0xFF12_3456));
    }
}

#[test]
fn halving_averages_each_pair() {
    // Columns alternate 0 and 200: every destination pixel covers one of each.
    let src: Vec<u32> = (0..8u32 * 4).map(|i| grey(if i % 2 == 0 { 0 } else { 200 })).collect();
    assert!(run(&src, 8, 4, 4, 2).iter().all(|&p| p == grey(100)));
}

#[test]
fn shrinking_by_one_and_a_half_weights_by_coverage() {
    // 3 cells to 2: the first covers cell 0 and half of cell 1.
    // 3 by 3 to 2 by 2 keeps the aspect; every row is the same.
    let line = [grey(0), grey(90), grey(240)];
    let src: Vec<u32> = line.iter().cycle().take(9).copied().collect();
    // (0 + 90 / 2) / 1.5 = 30, (90 / 2 + 240) / 1.5 = 190.
    assert_eq!(run(&src, 3, 3, 2, 2), vec![grey(30), grey(190), grey(30), grey(190)]);
}

#[test]
fn doubling_interpolates_between_centres() {
    let src = [grey(0), grey(200)];
    // Destination centres sit at 0.25 and 0.75 of the gap between cells.
    assert_eq!(run(&src, 2, 1, 4, 1), vec![grey(0), grey(50), grey(150), grey(200)]);
}

#[test]
fn other_aspect_crops_evenly_without_stretch() {
    // 16:9 source with a 2-pixel white stripe in the middle columns.
    let (sw, sh) = (32u32, 18u32);
    let src: Vec<u32> =
        (0..sw * sh).map(|i| grey(if (15..17).contains(&(i % sw)) { 255 } else { 0 })).collect();
    // 1:1 target: height is kept, 7 columns cut from each side.
    let out = run(&src, sw, sh, 18, 18);
    for y in 0..18 {
        let row = &out[y * 18..(y + 1) * 18];
        assert_eq!(row[8], grey(255));
        assert_eq!(row[9], grey(255));
        assert_eq!(row[7], grey(0));
        assert_eq!(row[10], grey(0));
    }
}

#[test]
fn short_source_is_refused() {
    assert!(!scale_cover(&[0u32; 10], 4, 4, 2, 2, |_, _| {}));
}
