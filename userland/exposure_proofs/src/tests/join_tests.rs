// NONOS Operating System (AGPL-3.0-or-later)
//! The join over seeded process tables and registries that overlap in
//! every way: in both, in the table alone, in the registry alone (exited).

use super::model::entry;
use crate::command::builtin::audit::join::join;
use std::collections::BTreeSet;

#[test]
fn every_capsule_in_either_list_is_one_row() {
    let (mut x, mut rows_seen, mut unsigned, mut exited) = (7u64, 0, 0, 0);
    for _ in 0..500 {
        let mut procs: Vec<(u32, Vec<u8>)> = vec![(0, b"kernel".to_vec())];
        let (mut regs, mut caps_of) = (Vec::new(), std::collections::BTreeMap::new());
        for pid in 1..40u32 {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let (listed, registered) = ((x >> 33) % 3 != 0, (x >> 40) % 3 != 0);
            if listed {
                procs.push((pid, format!("cap{pid}").into_bytes()));
            }
            if registered {
                regs.extend(entry(pid, x >> 20, (x >> 8) as u8));
                caps_of.insert(pid, x >> 20);
            }
        }
        let named: Vec<(u32, &[u8], bool)> =
            procs.iter().map(|(p, n)| (*p, &n[..], false)).collect();
        let rows = join(&regs, &named).unwrap();
        let want: BTreeSet<u32> =
            procs.iter().map(|p| p.0).filter(|p| *p != 0).chain(caps_of.keys().copied()).collect();
        let got: Vec<u32> = rows.iter().map(|r| r.pid).collect();
        assert_eq!(got, want.iter().copied().collect::<Vec<_>>(), "each once, by pid");
        for r in &rows {
            let listed = procs.iter().find(|p| p.0 == r.pid);
            assert_eq!(r.name.as_deref(), listed.map(|p| &p.1[..]));
            assert_eq!(r.ceiling.map(|c| c.caps), caps_of.get(&r.pid).copied());
            unsigned += usize::from(r.ceiling.is_none());
            exited += usize::from(r.name.is_none());
        }
        rows_seen += rows.len();
    }
    println!("{rows_seen} rows, {unsigned} without a ceiling, {exited} exited");
    assert!(rows_seen >= 13_800 && unsigned >= 3_400 && exited >= 3_500);
}
