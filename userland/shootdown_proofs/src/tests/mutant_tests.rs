// NONOS Operating System (AGPL-3.0-or-later)
//! Each rule of the round has a mutant that removes it, and the search finds
//! the counterexample: a rule no test can tell from its removal is unchecked.

use super::*;

#[test]
fn mutant_target_online_only() {
    // The code before this rule: an AP that woke and cached the old page
    // before the write, and was read as `Starting`, is skipped and keeps it.
    for n in CPUS {
        let cfg = Config { target_starting: false, ..round(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), STALE);
    }
}

#[test]
fn mutant_no_ack_for_abandoned() {
    // A target given up on never answers; nobody answering for it is a round
    // that waits to its deadline and stops the machine.
    for n in CPUS {
        let cfg = Config { ack_for_abandoned: false, ..round(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), "an ack is owed by a cpu that will never answer");
    }
}

#[test]
fn mutant_select_before_write() {
    for n in CPUS {
        let cfg = Config { write_before_select: false, ..round(n, 0, Flush::Page) };
        assert_eq!(broken(&cfg), STALE);
    }
}

#[test]
fn mutant_ack_before_flush() {
    for n in CPUS {
        let cfg = Config { flush_before_ack: false, ..round(n, n - 1, Flush::Page) };
        assert_eq!(broken(&cfg), STALE);
    }
}

#[test]
fn global_pages_survive_a_cr3_reload_on_a_pge_cpu() {
    // A full flush by CR3 write alone keeps the GLOBAL kernel entry on any CPU
    // with PGE set: the boot CPU alone, as firmware may leave it, or every CPU,
    // as the kernel now sets it. `invlpg` removes a global entry either way.
    for n in CPUS {
        for pge in [Pge::BootOnly, Pge::Every] {
            let bare = Config { pge, full_toggles_pge: false, ..round(n, 0, Flush::Full) };
            assert_eq!(broken(&bare), STALE);
            holds(&Config { pge, full_toggles_pge: false, ..round(n, 0, Flush::Page) });
        }
    }
}

#[test]
fn a_full_flush_that_toggles_pge_holds_however_pge_is_set() {
    for n in CPUS {
        for pge in [Pge::Off, Pge::BootOnly, Pge::Every] {
            holds(&Config { pge, ..round(n, 0, Flush::Full) });
        }
    }
}
