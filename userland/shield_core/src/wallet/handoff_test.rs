/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic, clippy::indexing_slicing)]

use super::{seal_outputs, write_handoff};
use crate::custody::{generate_phrase, phrase_to_seed};
use crate::keys::Account;
use crate::notes::{commitment, open_xwing, wire_digest, NotePlaintext, XwingOpened};
use crate::prover::launch::prove::LaunchProof;
use crate::prover::pool_hasher;

fn account() -> Account {
    Account::from_seed(&phrase_to_seed(&generate_phrase().expect("entropy")).expect("seed"))
        .expect("account")
}

fn note(value: u64, owner: &Account) -> NotePlaintext {
    NotePlaintext { value, asset_id: 1, blinding: [3, 1, 4, 1], spend_pk: owner.address().spend_pk }
}

fn proof(outputs: [NotePlaintext; 2]) -> LaunchProof {
    LaunchProof {
        bytes: vec![0xab; 4],
        publics: [7; 37],
        words: [[0; 32]; 13],
        outputs,
        grind_hashes: 0,
        weakened: Vec::new(),
        new_cache: None,
    }
}

#[test]
fn each_sealed_note_opens_for_its_owner_and_nobody_else() {
    let (me, payee) = (account(), account());
    let p = proof([note(1_000_000, &payee), note(2_000_000, &me)]);
    let sealed =
        seal_outputs(&p, &payee.receive().encapsulation_key(), &me.receive().encapsulation_key())
            .unwrap();
    let leaf =
        |n: &NotePlaintext| wire_digest(&commitment(&pool_hasher(), &n.note()).map(|f| f.value()));
    let opens = |who: &Account, i: usize| {
        matches!(
            open_xwing(
                &sealed[i],
                who.receive().dk(),
                &leaf(&p.outputs[i]),
                &who.address().spend_pk
            ),
            XwingOpened::Note(_)
        )
    };
    assert!(opens(&payee, 0), "the payee cannot open their note");
    assert!(opens(&me, 1), "the change does not come back to this wallet");
    assert!(!opens(&me, 0) && !opens(&payee, 1), "a note opened for someone it was not sealed to");
    let dir = std::env::temp_dir().join(format!("nox-handoff-{}", std::process::id()));
    let files = write_handoff(&dir, &p, &sealed).unwrap();
    let names: Vec<_> =
        files.iter().map(|f| f.file_name().unwrap().to_string_lossy().into_owned()).collect();
    assert_eq!(names, ["spend.proof", "spend.proof.publics.json", "blob0.bin", "blob1.bin"]);
    assert_eq!(std::fs::read(&files[2]).unwrap().len(), 1_186, "a sealed note is 1,186 bytes");
    std::fs::remove_dir_all(&dir).ok();
}
