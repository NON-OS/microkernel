//! What a lander's `/v1/info` says: the pool it serves, and whether it takes the submitter fee.

use super::field;
use crate::net::rpc::after;

/// Whether `/v1/info` names `pool` as the pool the lander serves.
pub(crate) fn serves(body: &str, pool: &str) -> bool {
    field(body, "pool").is_some_and(|p| p.eq_ignore_ascii_case(pool))
}

/// Whether `/v1/info` says the lander takes a proof whose fee goes to whoever submits it.
pub(crate) fn takes_submitter_fee(body: &str) -> bool {
    after(body, "\"fee_to_submitter_accepted\"")
        .map(|rest| rest.trim_start_matches([' ', ':']))
        .is_some_and(|rest| rest.starts_with("true"))
}
