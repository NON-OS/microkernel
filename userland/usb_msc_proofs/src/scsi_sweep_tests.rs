// NONOS Operating System (AGPL-3.0-or-later)
//! Sizing (capacity) and flushing (sync_cache) a disk over seeded devices,
//! honest and then told one lie: each ends as the rule in
//! scsi_sweep/model.rs says, using every answer the rule asks for and no
//! other, and as the lie must leave it.

use crate::bot_sweep::rng::{rounds, Rng};
use crate::descriptors::MscBinding;
use crate::disk::{capacity, sync_cache, Disk};
use crate::scsi_sweep::lie::{tell, Case, LIES};
use crate::scsi_sweep::model::Host;
use crate::state::State;
use crate::xhci::{device, script};

#[test]
fn after_any_lie_a_disk_is_sized_and_flushed_as_its_answers_say() {
    let b = MscBinding { interface: 0, bulk_in: 0x81, bulk_out: 0x02, ..MscBinding::default() };
    let mut told = vec![0u32; LIES.len()];
    for seed in 1..rounds(30_000) {
        let (lie, case, want) = tell(&mut Rng::new(seed));
        let at = format!("seed {seed}: {lie:?}");
        let mut host = Host { ignore: false, script: vec![] };
        let (disk, mut state) = (Disk::bound(1, 1, &b), State::default());
        let (model, got) = match &case {
            Case::Size(c10, c16, s) => {
                let model = host.capacity(c10, c16, s).map(Some);
                script(std::mem::take(&mut host.script));
                (model, capacity(&disk, &mut state).map(Some))
            }
            Case::Flush(cmd, s) => {
                let model = host.flush(cmd, s).map(|_| None);
                script(std::mem::take(&mut host.script));
                (model, sync_cache(&disk, &mut state).map(|_| None))
            }
        };
        assert_eq!(got, model, "{at}: the rule");
        assert_eq!(got, want, "{at}");
        assert_eq!(device(|d| d.answers.len()), 0, "{at}: answers the rule asked for, unread");
        told[LIES.iter().position(|l| *l == lie).unwrap()] += 1;
    }
    for (n, lie) in told.iter().zip(LIES) {
        assert!(cfg!(miri) || *n >= 1_500, "{lie:?} was told only {n} times");
    }
}
