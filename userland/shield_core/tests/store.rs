/// The stand ins this test runs against (tests/keystore/).
mod keystore {
    mod scratch;
    pub use scratch::scratch;
}
#[path = "store/assets.rs"]
mod assets;
#[path = "store/tamper.rs"]
mod tamper;

use keystore::scratch;
use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::net::asset::ETH;
use nox_shield_core::notes::{fresh_blinding, NotePlaintext, NoteRecord, NoteStatus};
use nox_shield_core::store::{NoteLog, Row};

fn record(value: u64, index: u64) -> NoteRecord {
    record_of(ETH.id, value, index)
}

fn record_of(asset_id: u64, value: u64, index: u64) -> NoteRecord {
    NoteRecord {
        plain: NotePlaintext {
            value,
            asset_id,
            blinding: fresh_blinding().expect("entropy"),
            spend_pk: [1, 2, 3, 4],
        },
        leaf_index: index,
        cm: [index, index + 1, index + 2, index + 3],
        status: NoteStatus::Unspent,
        found_at: index,
    }
}

#[test]
fn rows_survive_a_reopen_and_the_balance_follows_them() {
    let dir = scratch("store-reopen");
    let phrase = generate_phrase().expect("entropy");
    let seed = phrase_to_seed(&phrase).expect("seed");
    let path = dir.join("notes.log");

    let (mut log, _) = NoteLog::open(path.clone(), &seed).expect("open");
    for row in [Row::Found(record(1000, 0)), Row::Found(record(2000, 4)), Row::Cursor(8)] {
        log.append(&row).expect("append");
    }
    let spent = Row::Status { cm: [0, 1, 2, 3], status: NoteStatus::Spent };
    log.append(&spent).expect("append");

    let (_reopened, replayed) = NoteLog::open(path, &seed).expect("reopen");
    assert_eq!(replayed.balance(&ETH).spendable, 2000, "a spent note is not spendable");
    assert_eq!(replayed.balance(&ETH).note_count, 1);
    assert_eq!(replayed.cursor(), 8);
}
