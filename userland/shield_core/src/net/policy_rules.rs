//! AmountPolicy.rules(assetId), the public getter of `mapping(uint64 => Rule) rules` with
//! Rule { uint64 fee; uint8 minExp; uint8 maxExp; bool ranged; } (L1-Shield AmountPolicy.sol:67-72):
//! four words, read strictly, for showing the range only. Whether an amount is standard is the
//! pool's own call (isStandard, `policy_standard`); `standard` here mirrors it for the tests.

/// The largest exponent the policy allows (AmountPolicy's MAX_EXP).
pub const MAX_EXP: u8 = 19;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rule {
    /// The flat relay fee in note units, as relayFee() reports.
    pub fee: u64,
    pub min_exp: u8,
    pub max_exp: u8,
    /// Not ranged: no amount is standard.
    pub ranged: bool,
}

fn small(w: &[u8], bytes: usize) -> Option<u64> {
    let (high, low) = w.split_at(32 - bytes);
    high.iter().all(|b| *b == 0).then(|| low.iter().fold(0u64, |v, b| v << 8 | u64::from(*b)))
}

/// The four words, or none: a fee past u64, an exponent past MAX_EXP, minExp above maxExp, a bool
/// other than 0 or 1, or not exactly four words.
pub fn decode_rule(reply: &[u8]) -> Option<Rule> {
    if reply.len() != 128 {
        return None;
    }
    let w = |i: usize| reply.get(32 * i..32 * i + 32);
    let fee = small(w(0)?, 8)?;
    let (min_exp, max_exp) =
        (u8::try_from(small(w(1)?, 1)?).ok()?, u8::try_from(small(w(2)?, 1)?).ok()?);
    let ranged = match small(w(3)?, 1)? {
        0 => false,
        1 => true,
        _ => return None,
    };
    (max_exp <= MAX_EXP && min_exp <= max_exp).then_some(Rule { fee, min_exp, max_exp, ranged })
}

impl Rule {
    /// The pool's _standard: ranged, and 1, 2 or 5 times 10^k with minExp <= k <= maxExp.
    pub fn standard(&self, units: u64) -> bool {
        if !self.ranged || units == 0 {
            return false;
        }
        let (mut digit, mut k) = (units, 0u8);
        while digit % 10 == 0 {
            digit /= 10;
            k += 1;
        }
        matches!(digit, 1 | 2 | 5) && (self.min_exp..=self.max_exp).contains(&k)
    }

    /// The range a screen shows, in note units: 10^minExp to 5 x 10^maxExp.
    pub fn range(&self) -> (u128, u128) {
        (10u128.pow(u32::from(self.min_exp)), 5 * 10u128.pow(u32::from(self.max_exp)))
    }
}
