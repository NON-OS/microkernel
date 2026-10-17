// NONOS Operating System (AGPL-3.0-or-later)
//! T1 unloaded, then in seeded order a filler table and T1 again loaded
//! into the freed slots. The model, from what `mod.rs` says is kept: the
//! alias reads and writes nothing; the field over T1's region, and a
//! buffer field not yet used, work only while names of theirs are there;
//! a buffer field used before, and the kept element, read the old values; the kept references to
//! T1's name, to the alias and to a method's lost name are stale; and no
//! read ever returns a value of what took the slots. Seeds and floors are
//! our choice, from no source.

use super::fixture::dsdt;
use crate::consts::at;
use crate::exec::FaultKind;
use crate::rig::Rig;

fn get(r: &mut Rig, p: &str) -> Result<u64, FaultKind> {
    let n = |o: crate::object::Object| match o {
        crate::object::Object::Integer(v) => v,
        _ => 0,
    };
    r.call(p, vec![]).map(n).map_err(|f| f.kind)
}

#[test]
fn no_kept_path_reaches_a_removed_object() {
    // Scenes whose filler or reload took T1's freed slots; reads checked.
    let (mut reused, mut reads) = (0, 0);
    for seed in 0..64u64 {
        let mut r = Rig::new(&dsdt()).connected();
        r.probe.bytes.insert((1, 0x61), 0x5A);
        r.call("\\SETU", vec![]).unwrap();
        r.call("\\MKLC", vec![]).unwrap();
        let before = [
            get(&mut r, "\\RAV_"),
            get(&mut r, "\\RP0"),
            get(&mut r, "\\RP2"),
            get(&mut r, "\\RP3"),
        ];
        assert_eq!(before, [Ok(0x11), Ok(0x11), Ok(0x11), Err(FaultKind::Stale)], "seed {seed}");
        let bound = seed % 3 == 0;
        if bound {
            assert_eq!(get(&mut r, "\\RBX_"), Ok(0x33), "seed {seed}: first use binds");
        }
        let old = at(&r.aml, "\\D1.VAL").unwrap();
        r.call("\\UNLD", vec![]).unwrap();
        let steps = [["\\FILL", "\\RELD"], ["\\RELD", "\\FILL"]][(seed % 2) as usize];
        let n = (seed / 2 % 3) as usize;
        for s in &steps[..n.min(2)] {
            r.call(s, vec![]).unwrap();
        }
        reused += usize::from(r.aml.ns.get(old).is_some());
        let reloaded = at(&r.aml, "\\D1").is_some();
        let what = format!("seed {seed} after {:?}", &steps[..n.min(2)]);
        assert!(at(&r.aml, "\\AV").is_none(), "{what}: the alias went with its object");
        assert!(get(&mut r, "\\RAV_").is_err() && get(&mut r, "\\WAV_").is_err(), "{what}: alias");
        assert_eq!(get(&mut r, "\\RF2_").ok(), reloaded.then_some(0x5A), "{what}: field by name");
        let bfx = if bound || reloaded { Ok(0x33) } else { Err(FaultKind::Undefined) };
        assert_eq!(get(&mut r, "\\RBX_"), bfx, "{what}: buffer field");
        assert_eq!(get(&mut r, "\\RP1"), Ok(0x22), "{what}: element, detached");
        assert_eq!(get(&mut r, "\\WF2_").is_ok(), reloaded, "{what}: field write by name");
        for p in ["\\RP0", "\\RP2", "\\RP3", "\\WP0", "\\WP2", "\\WP3"] {
            assert_eq!(get(&mut r, p), Err(FaultKind::Stale), "{what}: {p}");
        }
        for p in ["\\RAV_", "\\RF2_", "\\RBX_", "\\RP0", "\\RP1", "\\RP2", "\\RP3"] {
            assert!(get(&mut r, p).map_or(true, |v| v < 0xF000), "{what}: {p} read a filler");
            reads += 1;
        }
    }
    assert!(reused >= 25 && reads == 64 * 7, "{reused} {reads}");
}
