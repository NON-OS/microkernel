// NONOS Operating System (AGPL-3.0-or-later)
//! The 3D emblem: a ring with its slash, a lit face, a side that falls only to
//! the lower right, edges blended into the theme, and plain blocks for a file.

use super::emblem::{emblem, pixel};
use super::emblem_cell::cell;
use super::emblem_shape::{cover, inside, DEPTH, EMBLEM_H, EMBLEM_W};

const DARK: [u32; 3] = [7, 9, 11];
const LIGHT: [u32; 3] = [255, 255, 255];

fn luma(c: [u32; 3]) -> u32 {
    c[0] * 3 + c[1] * 6 + c[2]
}

#[test]
fn it_is_a_ring_with_a_slash_through_it() {
    assert!(inside(18.5, 14.0), "the slash crosses the centre");
    assert!(inside(18.5 + 13.5, 14.0), "the ring at the right");
    assert!(inside(18.5, 14.0 - 10.0), "the ring at the top");
    assert!(!inside(18.5 + 6.0, 14.0 + 3.0), "the hole beside the slash is open");
    assert!(!inside(0.5, 0.5) && !inside(39.5, 29.5), "the corners are empty");
    assert!(inside(18.5 + 0.6428 * 15.0, 14.0 - 0.766 * 15.0), "the slash runs past the ring");
}

#[test]
fn the_side_falls_only_down_and_to_the_right() {
    for y in 0..EMBLEM_H as i32 {
        for x in 0..EMBLEM_W as i32 {
            if cover(x, y, 0.0) > 0 || pixel(x, y, DARK).is_none() {
                continue;
            }
            // Asked of the shape itself, not of `cover`: the face lies up and
            // to the left of every side pixel, 1.4 across for each step down.
            let reach = (1..=(DEPTH * 2.0) as i32).any(|s| {
                let k = s as f32 / 2.0;
                (0..16).any(|n| {
                    let (i, j) = ((n % 4) as f32 + 0.5, (n / 4) as f32 + 0.5);
                    inside(x as f32 + i / 4.0 - 1.4 * k, y as f32 + j / 4.0 - k)
                })
            });
            assert!(reach, "side at {x},{y} with no face up and to the left of it");
        }
    }
    let side = (0..EMBLEM_H as i32)
        .flat_map(|y| (0..EMBLEM_W as i32).map(move |x| (x, y)))
        .filter(|&(x, y)| cover(x, y, 0.0) == 0 && pixel(x, y, DARK).is_some())
        .count();
    assert!(side > 40, "a side deep enough to read as depth: {side} pixels");
}

#[test]
fn the_face_is_lit_from_the_upper_left_and_its_edges_are_smooth() {
    // Interior face pixels, clear of the lit edges: the first and last along
    // the light's direction.
    let mut inner: Vec<(i32, i32)> = (0..EMBLEM_H as i32)
        .flat_map(|y| (0..EMBLEM_W as i32).map(move |x| (x, y)))
        .filter(|&(x, y)| cover(x, y, 0.0) == 16 && cover(x - 1, y - 1, 0.0) == 16)
        .collect();
    inner.sort_by_key(|&(x, y)| 6 * x + 10 * y);
    let (a, b) = (inner[0], inner[inner.len() - 1]);
    let (lit, deep) = (pixel(a.0, a.1, DARK).unwrap(), pixel(b.0, b.1, DARK).unwrap());
    assert!(luma(lit) > luma(deep) + 300, "{lit:?} against {deep:?}");
    let partial = (0..EMBLEM_H as i32)
        .flat_map(|y| (0..EMBLEM_W as i32).map(move |x| cover(x, y, 0.0)))
        .filter(|c| (1..16).contains(c))
        .count();
    assert!(partial > 30, "edges are sampled, not stepped: {partial}");
}

#[test]
fn edges_blend_into_the_theme_and_the_face_does_not() {
    let bare = |x, y| (1..=(DEPTH * 2.0) as i32).all(|s| cover(x, y, s as f32 / 2.0) == 0);
    let (x, y) = (0..EMBLEM_H as i32)
        .flat_map(|y| (0..EMBLEM_W as i32).map(move |x| (x, y)))
        .find(|&(x, y)| (1..12).contains(&cover(x, y, 0.0)) && bare(x, y))
        .unwrap();
    assert_ne!(pixel(x, y, DARK), pixel(x, y, LIGHT), "an edge takes the background");
    assert_eq!(pixel(8, 8, DARK), pixel(8, 8, LIGHT), "a solid face pixel does not");
}

#[test]
fn rows_are_half_blocks_in_true_colour_and_plain_for_a_file() {
    let rows = emblem(0x07090B);
    assert_eq!(rows.len(), EMBLEM_H / 2);
    for (plain, styled) in &rows {
        assert_eq!(std::str::from_utf8(plain).unwrap().chars().count(), EMBLEM_W);
        assert!(!plain.contains(&0x1b));
        assert!(styled.windows(4).any(|w| w == b"38;2") || styled.iter().all(|&b| b == b' '));
    }
    let mut both = Vec::new();
    cell(&mut both, Some([1, 2, 3]), Some([4, 5, 6]));
    assert_eq!(both, "\x1b[38;2;1;2;3;48;2;4;5;6m▀\x1b[0m".as_bytes(), "two pixels a cell");
    let mut low = Vec::new();
    cell(&mut low, None, Some([4, 5, 6]));
    assert_eq!(low, "\x1b[38;2;4;5;6m▄\x1b[0m".as_bytes());
}
