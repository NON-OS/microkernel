/*
 * A live run against Sepolia, by hand, stage by stage. Tests assert by panicking.
 */

//! The whole private path on the launch pool, in the stages a person runs one at a time: the
//! addresses to fund, a deposit signed by account 1, a private send to account 2 through the
//! relayer over its onion, account 2 finding the note, and a withdrawal to a fresh public address.
//! The phrase comes from a public label, so it holds test money only and is no secret.
//! Run with `STAGE=addresses|shield|send|receive|withdraw` and `--ignored --nocapture`.

mod e2e;
/// The stand in this test runs against (tests/keystore/).
mod keystore {
    mod guard;
    pub use guard::TestGuard;
}

use e2e::{stage, wallet};

#[test]
#[ignore]
fn the_private_path_on_the_launch_pool() {
    let wallet = wallet();
    match std::env::var("STAGE").expect("a STAGE").as_str() {
        "addresses" => stage::addresses(&wallet),
        "shield" => stage::shield(&wallet),
        "send" => stage::send(&wallet),
        "receive" => stage::receive(&wallet),
        "withdraw" => stage::withdraw(&wallet),
        other => panic!("no stage {other}"),
    }
}
