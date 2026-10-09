//! A history read on from the one kept: the blocks under the kept head are read again, a few
//! dozen of them, so a tip the chain reorganised is replaced, and every log older than that is
//! kept as it was read. The whole is checked again after, as a first read is. Pure, so the rule
//! is tested without a network.

use crate::net::rpc::RawLog;

/// Blocks under the kept head read again: well past any reorganisation Sepolia or mainnet has
/// seen since the merge.
pub const REREAD: u64 = 64;

/// The first block a read on from a history kept at `kept_head` asks for.
pub fn read_from(kept_head: u64, deploy_block: u64) -> u64 {
    kept_head.saturating_sub(REREAD).max(deploy_block)
}

/// The kept logs older than `from`, then the fresh ones, which start at `from`.
pub fn joined(kept: &[RawLog], from: u64, fresh: Vec<RawLog>) -> Vec<RawLog> {
    let mut out: Vec<RawLog> = kept.iter().filter(|l| l.block < from).cloned().collect();
    out.extend(fresh);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log(block: u64, tag: u8) -> RawLog {
        RawLog { topics: vec![[tag; 32]], data: Vec::new(), block, tx: None }
    }

    fn blocks(logs: &[RawLog]) -> Vec<(u64, u8)> {
        logs.iter().map(|l| (l.block, l.topics[0][0])).collect()
    }

    #[test]
    fn the_tip_is_read_again_and_the_rest_kept() {
        let kept = [log(100, 1), log(500, 2), log(960, 3), log(1000, 4)];
        let from = read_from(1000, 50);
        assert_eq!(from, 936);
        /* The tip reorganised: the log at 960 went, one at 990 came. */
        let fresh = vec![log(990, 5), log(1010, 6)];
        assert_eq!(blocks(&joined(&kept, from, fresh)), [(100, 1), (500, 2), (990, 5), (1010, 6)]);
    }

    #[test]
    fn a_read_never_starts_before_the_pool_was_deployed() {
        assert_eq!(read_from(70, 50), 50);
        assert_eq!(read_from(0, 50), 50);
        assert_eq!(blocks(&joined(&[log(60, 1)], 50, vec![log(60, 1)])), [(60, 1)]);
    }
}
