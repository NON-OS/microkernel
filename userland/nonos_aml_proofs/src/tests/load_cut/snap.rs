// NONOS Operating System (AGPL-3.0-or-later)
//! What a later evaluation sees: every object's path and kind, and each
//! Name's value read with a full budget; and running \L (or the direct
//! load) under a step limit.

use super::fixture::{dsdt, ssdt};
use super::gen::Scene;
use crate::error::ErrorKind;
use crate::exec::{ExecLimits, Machine};
use crate::namespace::{write_path, Kind};
use crate::probe::Probe;
use crate::rig::Rig;
use std::collections::BTreeMap;

pub type Snap = BTreeMap<String, String>;

pub fn snap(r: &mut Rig) -> Snap {
    let mut out = Snap::new();
    for id in 0..r.aml.ns.len() as u32 {
        let Some(e) = r.aml.ns.get(id).copied() else { continue };
        let mut path = String::new();
        write_path(&r.aml.ns, id, &mut path).unwrap();
        let value = match e.kind {
            Kind::Name => format!("{:?}", r.call(&path, vec![])),
            k => format!("{k:?}"),
        };
        out.insert(path, value);
    }
    out
}

/// Loads under `steps`: whether it failed, the steps it took, and for a
/// direct load, the error it returned.
pub fn load(r: &mut Rig, sc: &Scene, steps: u64) -> (bool, u64, Option<ErrorKind>) {
    let lim = ExecLimits { steps, ..ExecLimits::DEFAULT };
    let mut m = Machine::new(&mut r.aml, &mut r.probe, lim);
    let (cut, error) = match sc.path {
        0 => {
            let e = m.load_table(&mut r.aml, ssdt(sc)).err().map(|e| e.kind);
            (e.is_some(), e)
        }
        _ => {
            let id = crate::consts::at(&r.aml, "\\L").unwrap();
            (m.evaluate(&mut r.aml, id, vec![]).is_err(), None)
        }
    };
    (cut, m.steps(), error)
}

/// The DSDT loaded, the table held for LoadTable, and the EC connected.
pub fn rig(sc: &super::gen::Scene) -> Rig {
    let probe = Probe { tables: vec![super::fixture::ssdt(sc)], ..Default::default() };
    let mut r = Rig::with(&dsdt(sc), probe);
    assert!(r.connect(3).is_empty());
    r
}
