use super::{seed_json, ROOM};
use crate::notes::NotePlaintext;

/// The widest seed file fits the room it was given, so no copy of the secret is left behind.
#[test]
fn the_seed_file_is_written_in_place() {
    let wide = NotePlaintext {
        value: u64::MAX,
        asset_id: u64::MAX,
        spend_pk: [u64::MAX; 4],
        blinding: [u64::MAX; 4],
    };
    let json = seed_json(&[u64::MAX; 4], [&wide, &wide]);
    assert_eq!(json.capacity(), ROOM);
    let small = NotePlaintext { value: 1, asset_id: 0, spend_pk: [2; 4], blinding: [3; 4] };
    let note = r#"{"value":1,"asset_id":0,"spend_pk":[2,2,2,2],"blinding":[3,3,3,3]}"#;
    let want = format!(r#"{{"secrets":[[9,9,9,9],[9,9,9,9]],"notes":[{note},{note}]}}"#);
    assert_eq!(*seed_json(&[9; 4], [&small, &small]), want);
}
