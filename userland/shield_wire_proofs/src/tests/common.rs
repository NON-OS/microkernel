// NONOS Operating System (AGPL-3.0-or-later)
//! The helpers every follow test frames its replies with.

use shield_wire::{decode_reply, encode_reply, Values, STATUS_OK};

use crate::pool::{look, Spend};
use crate::service_reply::{done, earlier, follow, Followed};

/// A reply as the wallet receives it, framed and unframed.
pub(super) fn carried(seq: u32, v: &Values) -> String {
    let bytes = encode_reply(seq, STATUS_OK, v.text());
    let r = decode_reply(&bytes).expect("the frame reads back");
    assert_eq!(r.seq, seq);
    String::from(r.body)
}

/// The RESULT a finished follow job answers, as the wallet reads it.
pub(super) fn follow_result(seq: u32, f: &Followed<'_>, kept: &[(&str, Followed<'_>)]) -> String {
    let mut job = Values::new();
    for (id, k) in kept {
        earlier(&mut job, id, k);
    }
    follow(&mut job, f);
    carried(seq, &done("follow", job.text()))
}

pub(super) fn spend() -> Spend {
    Spend {
        published_at: 0,
        last_published: 0,
        times: 1,
        lands_at: None,
        owner_lands_at: None,
        settling: false,
    }
}

/// A proved spend's publication, as the send job answers it.
pub(super) fn published(record: &str, refusal: Option<&str>) -> String {
    let mut job = Values::new();
    job.put("published", record);
    if let Some(why) = refusal {
        job.put("lander_refusal", why);
    }
    carried(9, &done("send", job.text()))
}

/// The mocked pool's spend at `now`, as the service follows it.
pub(super) fn at(s: &mut Spend, now: u32) -> Followed<'static> {
    let (state, minutes, self_settle, tx) = look(s, now);
    Followed { state, minutes, times: s.times, self_settle, tx, link: None, refusal: None }
}
