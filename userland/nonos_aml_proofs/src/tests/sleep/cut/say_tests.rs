// NONOS Operating System (AGPL-3.0-or-later)
//! F-120: every cut shutdown carried on through the capsule's own wording
//! (`capsule_acpi/src/sleep/shutdown_say.rs`, mounted once, in
//! `bringup_cut::capsule`) and the kernel's choice (`s5_choice.rs`). The model: the cut
//! method's line says it faulted, and where; the other's says it ran or is
//! absent; \_S5's line says its pair was handed over; and the kernel then
//! writes a pair whatever its scan found: its own when that fits, else the
//! capsule's.

#[allow(dead_code)]
#[path = "../../../../../../src/hardware/broker/acpi/s5_choice.rs"]
mod s5_choice;

use super::gen::scene;
use super::run::run;
use crate::exec::Fault;
use crate::tests::load_tests::bringup_cut::capsule::say::{method_line, s5_line, Part, Went};

fn text(write: impl FnOnce(&mut dyn FnMut(Part))) -> String {
    let mut s = String::new();
    write(&mut |p| match p {
        Part::Str(b) => s.push_str(core::str::from_utf8(b).unwrap()),
        Part::Hex(v) => s.push_str(&format!("{v:#x}")),
        Part::Dec(v) => s.push_str(&v.to_string()),
    });
    s
}

fn went(r: &Option<Result<(), Fault>>) -> Went {
    r.as_ref()
        .map_or(Went::Absent, |r| r.as_ref().map_or_else(|f| Went::Faulted(f.at), |_| Went::Ran))
}

#[test]
fn every_cut_shutdown_says_its_path_and_reaches_a_write() {
    let mut cuts = 0;
    for seed in 0..150 {
        let sc = scene(seed);
        let (.., used, _, _) = run(&sc, 1_000_000);
        for cut in 1..used {
            let (p, ..) = run(&sc, cut);
            let what = format!("seed {seed} cut at {cut}");
            let (tts, pts) = (went(&p.tts), went(&p.pts));
            let (cut_went, other) = if sc.cut_pts { (pts, tts) } else { (tts, pts) };
            let at = match cut_went {
                Went::Faulted(at) => at,
                w => panic!("{what}: {w:?}"),
            };
            let name: &[u8] = if sc.cut_pts { b"_PTS" } else { b"_TTS" };
            let line = text(|o| method_line(name, cut_went, o));
            assert!(line.ends_with(&format!("(5) faulted at {at:#x}")), "{what}: {line}");
            assert!(matches!(other, Went::Ran | Went::Absent), "{what}");
            let t = p.types.map(|t| (t.a, t.b)).expect("\\_S5 read");
            let line = text(|o| s5_line(Some(t), Ok(()), o));
            assert_eq!(line, format!("\\_S5 {}/{} handed to the kernel's soft-off", t.0, t.1));
            for scan in [None, Some((7, 7)), Some((9, 9))] {
                let wrote = s5_choice::choose(scan, Some(t)).map(|w| w.0);
                assert_eq!(wrote, Some(scan.filter(|s| s.0 <= 7).unwrap_or(t)), "{what}");
            }
            cuts += 1;
        }
    }
    assert!(cuts >= 3500, "{cuts}");
}
