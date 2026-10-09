// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Post-layout effects and box geometry: transforms map what a box painted,
//! clip-path cuts it to its shape's bounds, aspect-ratio sizes an auto
//! height, and corner radii resolve per corner and by percentage.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn a_zero_scale_hides_the_box_and_its_content() {
    let html = "<div id=p style=\"height:2px;transform:scaleX(0);background:#66ffff\">x</div>";
    let p = Page::at(html, VP);
    assert_eq!(p.frag("p").alpha, 0);
    assert_eq!(p.word("x").alpha, 0);
}

#[test]
fn a_percentage_translate_moves_by_the_boxs_own_size() {
    let html = "<body style=\"margin:0\"><div id=d style=\"width:100px;height:40px;\
                margin-left:200px;transform:translate(-50%, -50%)\"></div></body>";
    let r = Page::at(html, VP).rect("d");
    assert_eq!((r[0], r[1], r[2], r[3]), (150, -20, 100, 40));
}

#[test]
fn a_scale_about_the_top_origin_keeps_the_top_edge() {
    let html = "<body style=\"margin:0\"><div id=d style=\"width:100px;height:100px;\
                transform-origin:top;transform:scaleY(0.5)\"></div></body>";
    assert_eq!(Page::at(html, VP).rect("d"), [0, 0, 100, 50]);
}

#[test]
fn a_circle_of_radius_zero_clips_everything_away() {
    let html = "<div id=m style=\"height:300px;clip-path:circle(0 at 90% 10%)\">menu</div>";
    assert_eq!(Page::at(html, VP).frag("m").alpha, 0);
}

#[test]
fn an_inset_clip_path_cuts_to_the_inner_rectangle() {
    let html = "<body style=\"margin:0\"><div id=d style=\"width:100px;height:100px;\
                clip-path:inset(10px 20% 30px 0)\"></div></body>";
    assert_eq!(Page::at(html, VP).frag("d").clip, Some([0, 10, 80, 70]));
}

#[test]
fn aspect_ratio_derives_an_auto_height_from_the_width() {
    let html = "<body style=\"margin:0\"><div id=c style=\"width:1196px;aspect-ratio:16 / 9\"></div></body>";
    assert_eq!(Page::at(html, (1336, 760)).rect("c")[3], 673);
}

#[test]
fn a_percentage_radius_rounds_a_square_into_a_circle() {
    let html = "<div id=d style=\"width:10px;height:10px;border-radius:50%\"></div>";
    assert_eq!(Page::at(html, VP).frag("d").radius, [5; 4]);
}

#[test]
fn corner_radii_are_kept_per_corner() {
    let html = "<div id=d style=\"width:100px;height:50px;border-radius:1px 2px 3px 4px\"></div>";
    assert_eq!(Page::at(html, VP).frag("d").radius, [1, 2, 3, 4]);
}
