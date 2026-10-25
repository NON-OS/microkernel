//! Received, deposited and spent: each of the three events finds what it should.

use super::held::held_and_spent;
use super::{abi_bytes, account, index_topic, leaf_of, paid_to, EV};
use nox_shield_core::discovery::{scan_chain, Log};
use nox_shield_core::notes::NotePlaintext;

#[test]
fn received_deposited_and_spent_are_all_found() {
    let me = account();
    let stranger = account();
    let (to_me, blob_me) = paid_to(&me, 500);
    let (_to_them, blob_them) = paid_to(&stranger, 900);
    // A deposit this wallet sent, and one it sent that the pool has not stored.
    let deposit = NotePlaintext {
        value: 3_000,
        asset_id: 1,
        blinding: [5, 5, 5, 5],
        spend_pk: me.address().spend_pk,
    };
    let waiting = NotePlaintext {
        value: 7_000,
        asset_id: 1,
        blinding: [6, 6, 6, 6],
        spend_pk: me.address().spend_pk,
    };

    let committed_topics = [
        [EV, leaf_of(&deposit), index_topic(0)],
        [EV, leaf_of(&to_me), index_topic(1)],
        [EV, [0x77; 32], index_topic(2)],
    ];
    let committed: Vec<Log> =
        committed_topics.iter().map(|t| Log { topics: t, data: &[] }).collect();
    let out_me = abi_bytes(&blob_me);
    let out_them = abi_bytes(&blob_them);
    let out_topics = [[EV, index_topic(1)], [EV, index_topic(2)]];
    let outputs = vec![
        Log { topics: &out_topics[0], data: &out_me },
        Log { topics: &out_topics[1], data: &out_them },
    ];

    // A note this wallet held earlier, now spent on chain.
    let (held, words, nf_topic) = held_and_spent(&me);
    let nf_topics = [nf_topic];
    let nullifiers = vec![Log { topics: &nf_topics[0], data: &[] }];

    let scan = scan_chain(
        &me,
        &committed,
        &outputs,
        &nullifiers,
        &[deposit, waiting],
        &held.iter().collect::<Vec<_>>(),
    );
    assert_eq!(scan.received.len(), 1, "the note paid to us, and not the stranger's");
    assert_eq!((scan.received[0].plain.value, scan.received[0].leaf_index), (500, 1));
    assert_eq!(scan.deposited.len(), 1, "the stored deposit, and not the waiting one");
    assert_eq!((scan.deposited[0].plain.value, scan.deposited[0].leaf_index), (3_000, 0));
    assert_eq!(scan.spent, vec![words], "the held note's nullifier was published");
}
