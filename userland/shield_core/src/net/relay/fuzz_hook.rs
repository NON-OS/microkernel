//! The replies of the lander a fuzzer reaches. Built only with `fuzzing`.

pub(crate) fn replies(status: u16, body: &str) {
    let _ = super::reply::handed(status, body);
    let _ = super::reply::state(body);
    let _ = super::reply::takes_submitter_fee(body);
}
