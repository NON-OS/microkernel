// NONOS Operating System (AGPL-3.0-or-later)
use super::*;

#[test]
fn a_request_reads_back_as_it_was_written() {
    let bytes = encode_request(7, OP_SEND, &["ETH", "nox1abc", "0.02", "0"]);
    let r = decode_request(&bytes).unwrap();
    assert_eq!((r.seq, r.op), (7, OP_SEND));
    let f: alloc::vec::Vec<&str> = r.body.split('\n').collect();
    assert_eq!(f, ["ETH", "nox1abc", "0.02", "0"]);
}

#[test]
fn a_reply_reads_back_with_its_status_and_values() {
    let mut v = Values::new();
    v.put("state", "done").put("tx", "0x01").put("balance", "ETH 0.1").put("balance", "NOX 5");
    let bytes = encode_reply(9, STATUS_OK, v.text());
    let r = decode_reply(&bytes).unwrap();
    assert_eq!((r.seq, r.status), (9, STATUS_OK));
    assert_eq!(field(r.body, "tx"), Some("0x01"));
    let all: alloc::vec::Vec<&str> = fields(r.body, "balance").collect();
    assert_eq!(all, ["ETH 0.1", "NOX 5"]);
    assert_eq!(field(r.body, "missing"), None);
}

#[test]
fn a_value_cannot_forge_another_line() {
    let mut v = Values::new();
    v.put("why", "bad\nstate=done");
    assert_eq!(field(v.text(), "state"), None);
    assert_eq!(field(v.text(), "why"), Some("bad state=done"));
}

#[test]
fn a_short_or_unreadable_message_is_refused() {
    assert!(decode_request(&[1, 2, 3]).is_none());
    assert!(decode_reply(&[0, 0, 0, 0, 0, 0, 0, 0, 0xff]).is_none());
}

#[test]
fn a_finished_job_keeps_its_own_state_under_the_envelope() {
    let mut job = Values::new();
    job.put("state", "landed").put("tx", "0x02");
    let mut v = Values::new();
    v.put("job", "follow").put("state", "done").put_job(job.text());
    assert_eq!(field(v.text(), "state"), Some("done"));
    assert_eq!(field(v.text(), "job"), Some("follow"));
    assert_eq!(field(v.text(), "job_state"), Some("landed"));
    assert_eq!(field(v.text(), "tx"), Some("0x02"));
}

#[test]
fn a_long_list_keeps_its_newest_that_fit() {
    let items: alloc::vec::Vec<alloc::string::String> =
        (0..10).map(|i| alloc::format!("entry {i}")).collect();
    let mut v = Values::new();
    /* Each line is "e=entry N\n", 10 bytes: room for three. */
    let left = v.put_newest("e", &items, 35);
    assert_eq!(left, 7);
    let kept: alloc::vec::Vec<&str> = fields(v.text(), "e").collect();
    assert_eq!(kept, ["entry 7", "entry 8", "entry 9"]);
    assert!(v.text().len() <= 35);
    let mut all = Values::new();
    assert_eq!(all.put_newest("e", &items, 1000), 0);
    assert_eq!(fields(all.text(), "e").count(), 10);
}
