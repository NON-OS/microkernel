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
    // With CR4.PGE set and GLOBAL kernel PTEs, a full flush by CR3 reload
    // keeps the stale entry; `invlpg` removes a global entry and is safe.
    for n in CPUS {
        let full = Config { initiator_pge: true, ..round(n, 0, Flush::Full) };
        assert_eq!(broken(&full), STALE);
        holds(&Config { initiator_pge: true, ..round(n, 0, Flush::Page) });
    }
}
