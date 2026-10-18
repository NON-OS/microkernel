// NONOS Operating System (AGPL-3.0-or-later)
//! One shutdown prepared with \_TTS or \_PTS held to `cut` steps and
//! everything else given the full budget: what prepare returned, the
//! Debug markers, the steps the cut method used, and what was left.

use super::fixture::source;
use super::gen::Scene;
use crate::consts::at;
use crate::exec::{ExecLimits, Machine};
use crate::load::Aml;
use crate::probe::Probe;
use crate::run::boot;
use crate::sleep::{prepare, Prepared};

pub fn run(sc: &Scene, cut: u64) -> (Prepared, Vec<String>, u64, Aml, Probe) {
    let (mut aml, _) = boot(&source(sc), 2);
    let mut probe = Probe { facs: Some(0), ..Default::default() };
    let target = at(&aml, if sc.cut_pts { "\\_PTS" } else { "\\_TTS" });
    let mut used = 0;
    let p = {
        let mut eval = |aml: &mut Aml, id, args| {
            let steps = if Some(id) == target { cut } else { 1_000_000 };
            let mut m = Machine::new(aml, &mut probe, ExecLimits { steps, ..ExecLimits::DEFAULT });
            let r = m.evaluate(aml, id, args);
            if Some(id) == target {
                used = m.steps();
            }
            r
        };
        prepare(&mut aml, &mut eval, 5)
    };
    (p, probe.debug.clone(), used, aml, probe)
}

pub fn group(d: &[String], tens: u8) -> Vec<String> {
    d.iter().filter(|s| s.starts_with(&format!("Integer({tens}"))).cloned().collect()
}
