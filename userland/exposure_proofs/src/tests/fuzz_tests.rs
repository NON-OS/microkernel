// NONOS Operating System (AGPL-3.0-or-later)
//! The registry and process-table readers fed what a broken or hostile
//! kernel answer could hold: registries of any length and content, names
//! of any bytes and length, repeated pids, pid 0. Nothing panics, a ragged
//! registry is refused whole, every listed pid is one row, and every line
//! the terminal would print is plain printable ASCII.

use crate::command::builtin::audit::agent::agent;
use crate::command::builtin::audit::join::{join, ENTRY_LEN};
use crate::command::builtin::audit::table::line;

fn next(x: &mut u64) -> u64 {
    *x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    *x >> 16
}

#[test]
fn hostile_tables_never_panic_or_print_control_bytes() {
    let (mut x, mut refused, mut rows_seen) = (99u64, 0, 0);
    for _ in 0..30_000 {
        // Half whole entries, half any length.
        let n = (next(&mut x) % 400) as usize;
        let n = if next(&mut x).is_multiple_of(2) { n - n % ENTRY_LEN } else { n };
        let regs: Vec<u8> = (0..n).map(|_| next(&mut x) as u8).collect();
        let procs_owned: Vec<(u32, Vec<u8>)> = (0..next(&mut x) % 12)
            .map(|_| {
                let pid = (next(&mut x) % 6) as u32;
                (pid, (0..next(&mut x) % 40).map(|_| next(&mut x) as u8).collect())
            })
            .collect();
        let procs: Vec<(u32, &[u8], bool)> =
            procs_owned.iter().map(|(p, n)| (*p, &n[..], n.len() % 2 == 1)).collect();
        let Some(rows) = join(&regs, &procs) else {
            assert!(!n.is_multiple_of(ENTRY_LEN), "a whole registry is never refused");
            refused += 1;
            continue;
        };
        assert!(n.is_multiple_of(ENTRY_LEN));
        for &(pid, _, _) in procs.iter().filter(|p| p.0 != 0) {
            assert!(rows.iter().any(|r| r.pid == pid), "listed pid {pid} kept");
        }
        for r in &rows {
            let l = line(r);
            assert!(l.iter().all(|b| (0x20..0x7f).contains(b)), "{l:?}");
            if let Some(name) = &r.name {
                for a in agent(&rows, name) {
                    assert!(a.iter().all(|b| (0x20..0x7f).contains(b)), "{a:?}");
                }
            }
        }
        rows_seen += rows.len();
    }
    println!("{refused} ragged registries refused, {rows_seen} rows printed safe");
    assert!(refused >= 11_700 && rows_seen >= 82_700, "{refused} {rows_seen}");
}
