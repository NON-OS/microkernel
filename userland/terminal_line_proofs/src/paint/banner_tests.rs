// NONOS Operating System (AGPL-3.0-or-later)
//! The splash banner is drawn from its blocks alone: every lit pixel lies on
//! a block or the side behind one, the face is lit from above, the side is
//! darker, and a cell that holds only a box stroke stays background.

use super::fetch_banner::{draw_banner, BANNER};
use crate::theme::profiles::DARK;
use nonos_app_skeleton::PaintBuffer;

const W: u32 = 420;
const H: u32 = 120;

fn drawn() -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    let below = draw_banner(&mut fb, 4, 4, W as i32, &DARK);
    assert_eq!(below, 4 + 6 * 15 + 3);
    px
}

fn block_at(x: u32, y: u32, depth: i32) -> bool {
    let (cx, cy) = ((x as i32 - 4 - depth) / 8, (y as i32 - 4 - depth) / 15);
    let lines: Vec<Vec<char>> = BANNER.iter().map(|l| l.chars().collect()).collect();
    x as i32 >= 4 + depth && y as i32 >= 4 + depth
        && lines.get(cy as usize).and_then(|l| l.get(cx as usize)) == Some(&'\u{2588}')
}

#[test]
fn only_blocks_and_their_sides_are_drawn() {
    let px = drawn();
    for y in 0..H {
        for x in 0..W {
            if px[(y * W + x) as usize] != 0 {
                assert!(block_at(x, y, 0) || block_at(x, y, 3), "stray pixel at {x},{y}");
            }
        }
    }
}

#[test]
fn the_face_is_lit_from_above_and_the_side_is_darker() {
    let px = drawn();
    let luma = |c: u32| (c >> 16 & 0xFF) * 3 + (c >> 8 & 0xFF) * 6 + (c & 0xFF);
    // The N's first column: top row and bottom row of the face, and its side.
    let (top, bottom) = (px[(5 * W + 5) as usize], px[(70 * W + 5) as usize]);
    assert!(luma(top) > luma(bottom), "{top:#x} over {bottom:#x}");
    let side = px[(81 * W + 21) as usize];
    assert!(side != 0 && luma(side) * 2 < luma(bottom), "side {side:#x}");
    // Where the side of the block above meets a face, the face is on top.
    let met = px[(20 * W + 8) as usize];
    assert!(luma(met) > luma(side) * 2, "the face, not the side, at 8,20: {met:#x}");
}

#[test]
fn a_box_stroke_cell_stays_background() {
    let px = drawn();
    // Row 0, column 3 holds a box stroke and nothing drawn reaches it.
    let (x, y) = (4 + 3 * 8 + 5, 4 + 1);
    assert_eq!(px[(y * W + x) as usize], 0);
}
