use super::*;

fn row(kind: u8, f: impl FnOnce(&mut Activity)) -> Activity {
    let mut a = Activity::new(kind, 100);
    f(&mut a);
    a
}

#[test]
fn a_deposit_is_on_its_way_until_its_note_is_stored() {
    let sent = row(DEPOSIT_SENT, |a| {
        a.value = 5;
        a.cm = [1, 2, 3, 4];
        a.tx = [9; 32];
    });
    let h = history(&[sent]);
    assert_eq!(h.len(), 1);
    assert!(h.iter().all(|e| e.stage == Stage::OnItsWay && e.tx == Some([9; 32])));
    let stored = row(DEPOSIT_IN, |a| a.cm = [1, 2, 3, 4]);
    let h = history(&[sent, stored]);
    assert!(h.iter().all(|e| e.stage == Stage::Done));
    let other = row(DEPOSIT_IN, |a| a.cm = [7, 7, 7, 7]);
    assert!(history(&[sent, other]).iter().all(|e| e.stage == Stage::OnItsWay));
}

#[test]
fn a_spend_lands_or_is_settled_or_taken_back_by_its_nullifier() {
    let sent = row(SENT, |a| {
        a.value = 3;
        a.tag = [1; 32];
    });
    let other = row(SENT, |a| a.tag = [2; 32]);
    let landed = row(LANDED, |a| {
        a.tag = [1; 32];
        a.tx = [5; 32];
    });
    let h = history(&[sent, other, landed]);
    let stages: Vec<Stage> = h.iter().map(|e| e.stage).collect();
    assert_eq!(stages, [Stage::Done, Stage::OnItsWay], "only the spend it names lands");
    assert_eq!(h.first().and_then(|e| e.tx), Some([5; 32]));
    let settle = row(SETTLED_SELF, |a| {
        a.tag = [2; 32];
        a.tx = [6; 32];
    });
    let h = history(&[sent, other, settle]);
    assert_eq!(h.get(1).map(|e| e.stage), Some(Stage::Settling));
    let back = row(TAKEN_BACK, |a| a.tag = [2; 32]);
    let h = history(&[sent, other, back, landed]);
    let stages: Vec<Stage> = h.iter().map(|e| e.stage).collect();
    assert_eq!(stages, [Stage::Done, Stage::TakenBack]);
}

#[test]
fn a_landed_spend_is_never_taken_back() {
    let sent = row(SENT, |a| a.tag = [1; 32]);
    let landed = row(LANDED, |a| a.tag = [1; 32]);
    let back = row(TAKEN_BACK, |a| a.tag = [1; 32]);
    let h = history(&[sent, landed, back]);
    assert!(h.iter().all(|e| e.stage == Stage::Done));
}

#[test]
fn change_is_told_apart_from_a_payment_received() {
    let sent = row(SENT, |a| {
        a.tag = [1; 32];
        a.cm = [4, 4, 4, 4];
    });
    assert!(is_change(&[sent], &[4, 4, 4, 4]));
    assert!(!is_change(&[sent], &[5, 5, 5, 5]));
    let landed = row(LANDED, |a| a.tag = [1; 32]);
    assert!(!settled(&[sent], &[1; 32]));
    assert!(settled(&[sent, landed], &[1; 32]));
}

#[test]
fn a_hash_survives_the_words_a_row_carries() {
    let mut b = [0u8; 32];
    for (i, x) in b.iter_mut().enumerate() {
        *x = u8::try_from(i).unwrap_or(0).wrapping_mul(7);
    }
    assert_eq!(from_quad(&to_quad(&b)), b);
}

#[test]
fn an_activity_row_reads_back_and_an_unknown_kind_is_refused() {
    let a = row(LANDED, |a| {
        a.asset_id = 2;
        a.value = 77;
        a.tx = [3; 32];
        a.tag = [4; 32];
        a.cm = [5, 6, 7, 8];
    });
    let bytes = crate::store::Row::Activity(a).encode();
    let back = crate::store::row::decode(&bytes);
    assert!(matches!(back, Ok(crate::store::Row::Activity(b)) if b == a));
    let mut odd = bytes.clone();
    if let Some(k) = odd.get_mut(1) {
        *k = 99;
    }
    assert!(crate::store::row::decode(&odd).is_err(), "an activity this build does not know");
    let mut short = bytes;
    short.pop();
    assert!(crate::store::row::decode(&short).is_err(), "a row cut short");
}
