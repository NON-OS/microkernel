// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's bring-up order (`boot/bringup.rs`): connect each space
//! (with _REG), \_PIC(1), _OSC on \_SB and the host bridge, then _INI.
//! One phase may be cut: each evaluation in it gets `steps` and no more,
//! as the capsule gives each evaluation its own budget. Returns what each
//! phase gave, the capsule's lines for it, and the most steps any one
//! evaluation in each phase used.

use super::capsule::bringup_say::{ini_line, pic_line, reg_line};
use super::capsule::text;
use crate::exec::{ExecLimits, Fault, Machine};
use crate::load::Aml;
use crate::namespace::{Id, ROOT};
use crate::object::Object;
use crate::platform::{initialize, negotiate, set_pic, PCI_HOST_UUID, PLATFORM_UUID};
use crate::probe::Probe;
use crate::region::Space;
use crate::rig::Rig;

pub struct Done {
    pub reg_failed: usize,
    pub pic: Option<Result<(), u32>>,
    pub osc: [bool; 2],
    pub ini: [usize; 4],
    pub lines: Vec<String>,
    pub used: [u64; 4],
}

pub fn bring_up(r: &mut Rig, cut: Option<(usize, u64)>) -> Done {
    let lim = |p: usize| match cut {
        Some((q, steps)) if q == p => steps,
        _ => 1_000_000,
    };
    let (mut used, mut lines, mut reg_failed) = ([0u64; 4], Vec::new(), 0);
    for s in 0..4u8 {
        let mut m = Machine::new(
            &mut r.aml,
            &mut r.probe,
            ExecLimits { steps: lim(0), ..ExecLimits::DEFAULT },
        );
        let failed = m.connect(&mut r.aml, Space(s)).len();
        used[0] = used[0].max(m.steps());
        reg_failed += failed;
        lines.push(text(|o| reg_line(failed, s, o)));
    }
    let aml = &mut r.aml;
    let pic = set_pic(aml, &mut eval_with(&mut r.probe, lim(1), &mut used[1]), 1);
    let pic = pic.map(|r| r.map_err(|f| f.at));
    lines.push(text(|o| pic_line(pic, o)));
    let sb = aml.ns.child(ROOT, *b"_SB_").unwrap();
    let pci = aml.ns.child(sb, *b"PCI0").unwrap();
    let (posc, hosc) = (aml.ns.child(sb, *b"_OSC").unwrap(), aml.ns.child(pci, *b"_OSC").unwrap());
    let mut eval = eval_with(&mut r.probe, lim(2), &mut used[2]);
    let platform = negotiate(aml, &mut eval, posc, (&PLATFORM_UUID, 1), &[0, 0]).is_ok();
    let host = negotiate(aml, &mut eval, hosc, (&PCI_HOST_UUID, 1), &[0, 0x1F, 0x1D]).is_ok();
    drop(eval);
    let rep = initialize(aml, &mut eval_with(&mut r.probe, lim(3), &mut used[3]));
    let failed = rep.ran.iter().filter(|(_, x)| x.is_err()).count();
    let ini = [rep.ran.len(), failed, rep.absent.len(), rep.sta_failed.len()];
    lines.push(text(|o| ini_line(ini, o)));
    Done { reg_failed, pic, osc: [platform, host], ini, lines, used }
}

fn eval_with<'a>(
    probe: &'a mut Probe,
    steps: u64,
    used: &'a mut u64,
) -> impl FnMut(&mut Aml, Id, Vec<Object>) -> Result<Object, Fault> + 'a {
    move |aml, id, args| {
        let mut m = Machine::new(aml, &mut *probe, ExecLimits { steps, ..ExecLimits::DEFAULT });
        let got = m.evaluate(aml, id, args);
        *used = (*used).max(m.steps());
        got
    }
}
