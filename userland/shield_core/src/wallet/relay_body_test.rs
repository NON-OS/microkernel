/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used)]

use super::{body, limbs};
use crate::notes::BLOB_LEN;

fn handoff(proof: usize, blob: usize, limbs: usize) -> std::path::PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("relay-body-{proof}-{blob}-{limbs}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("spend.proof"), vec![7u8; proof]).unwrap();
    std::fs::write(dir.join("blob0.bin"), vec![1u8; blob]).unwrap();
    std::fs::write(dir.join("blob1.bin"), vec![2u8; blob]).unwrap();
    let list: Vec<String> = (0..limbs).map(|i| i.to_string()).collect();
    std::fs::write(
        dir.join("spend.proof.publics.json"),
        format!("{{\"publics\": [{}]}}\n", list.join(", ")),
    )
    .unwrap();
    dir
}

#[test]
fn a_whole_hand_off_becomes_the_body_the_lander_reads() {
    let json = body(&handoff(95_208, BLOB_LEN, 37)).unwrap();
    assert!(json.starts_with(r#"{"proof":"BwcH"#));
    assert!(json.contains(r#""publics":[0,1,2,"#));
    assert!(json.ends_with(r#""}"#));
}

#[test]
fn a_hand_off_of_the_wrong_shape_is_refused_before_it_leaves() {
    assert!(body(&handoff(79_999, BLOB_LEN, 37)).is_err(), "a short proof");
    assert!(body(&handoff(110_001, BLOB_LEN, 37)).is_err(), "a long proof");
    assert!(body(&handoff(95_208, BLOB_LEN - 1, 37)).is_err(), "a short note");
    assert!(
        body(&handoff(95_208, BLOB_LEN, 36)).is_err(),
        "36 limbs, the statement before not-before"
    );
    assert_eq!(limbs(r#"{"publics": [1, 18446744073709551616]}"#), None, "a limb past 64 bits");
}
