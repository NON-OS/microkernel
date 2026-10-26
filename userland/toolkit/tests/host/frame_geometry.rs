#[path = "../../src/decorations/accessory.rs"]
mod accessory;
#[path = "../../src/decorations/control_rect.rs"]
mod control_rect;
#[path = "../../src/decorations/frame_rect.rs"]
mod frame_rect;
#[path = "../../src/decorations/hit_test.rs"]
mod hit_test;
#[path = "../../src/decorations/inner_rect.rs"]
mod inner_rect;
#[path = "../../src/decorations/metrics.rs"]
mod metrics;
#[path = "../../src/decorations/rect.rs"]
mod rect;
#[path = "../../src/decorations/scale.rs"]
mod scale;

use accessory::*;
use control_rect::*;
use frame_rect::*;
use hit_test::*;
use inner_rect::*;
use metrics::*;

fn main() {
    let (w, h) = (560u32, 360u32);
    let f = frame_rect(w, h, false);
    assert_eq!((f.x, f.y, f.w, f.h), (10, 10, 540, 340), "frame_rect");
    let c = content_rect(w, h, false);
    assert_eq!((c.x, c.y, c.w, c.h), (11, 46, 538, 303), "content_rect");
    let t = titlebar_rect(w, h, false);
    assert_eq!((t.x, t.y, t.w, t.h), (10, 10, 540, 36), "titlebar_rect");
    for (i, ex) in [438u32, 474, 510].iter().enumerate() {
        let r = control_rect(i as u32, w, h, false);
        assert_eq!((r.x, r.y, r.w, r.h), (*ex, 14, 36, 28), "control {}", i);
    }
    let fm = frame_rect(w, h, true);
    assert_eq!((fm.x, fm.y, fm.w, fm.h), (0, 0, w, h), "maximized frame");
    assert_eq!((margin(true), radius(true)), (0, 0));
    let cm = content_rect(w, h, true);
    assert_eq!((cm.x, cm.y, cm.w, cm.h), (1, 36, w - 2, h - 37), "maximized content");
    for (tw, th) in
        [(0u32, 0u32), (1, 1), (20, 20), (21, 41), (2 * SHADOW_MARGIN, 2 * SHADOW_MARGIN)]
    {
        let c = content_rect(tw, th, false);
        assert!(c.x + c.w <= tw.max(c.x), "content escapes surface at {}x{}", tw, th);
        let f = frame_rect(tw, th, false);
        assert!(f.w <= tw && f.h <= th, "frame escapes surface at {}x{}", tw, th);
    }
    let inside = frame_rect(w, h, false);
    assert!(!inside.contains(9, 9) && inside.contains(10, 10) && !inside.contains(550, 350));
    assert!(TITLEBAR_H >= CONTROL_H && TITLEBAR_H >= TITLE_PX as u32, "bar too short");
    assert!(accessory_rect(w, h, false, 0).is_none(), "zero width must claim no titlebar");
    let a = accessory_rect(w, h, false, 236).expect("accessory fits a 560px window");
    assert_eq!((a.x, a.y, a.w, a.h), (190, 17, 236, 22), "accessory_rect");
    assert!(t.contains(a.x, a.y) && t.contains(a.x + a.w - 1, a.y + a.h - 1), "escapes the bar");
    assert!(a.x + a.w + ACCESSORY_INSET <= controls_left_at(w, h, false, 4), "over the controls");
    assert!(accessory_rect(60, h, false, 236).is_none(), "no room beside the controls at 60");
    let hits = [(528, 28, DecorationHit::CloseButton), (300, 28, DecorationHit::Titlebar)];
    for (x, y, want) in hits.into_iter().chain([(300, 100, DecorationHit::None)]) {
        assert_eq!(hit_test(w, h, false, x, y), want, "hit at {},{}", x, y);
    }
    assert_eq!(chrome_growth_at(4), (0, 0), "no growth at one to one");
    println!("[FRAME-GEOM] PASS");
}
