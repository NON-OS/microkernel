//! The pool's revert reasons as sentences, keyed by selector. Anything else shows its selector.

const SENTENCES: &[([u8; 4], &str)] = &[
    ([0xb4, 0x21, 0x1f, 0x16], "The pool no longer knows the root this proof was made against."),
    ([0x89, 0x64, 0xa3, 0x8a], "The association set this proof names is not registered."),
    ([0xb1, 0x15, 0xd8, 0x57], "One of these notes is already spent."),
    ([0xe1, 0x20, 0x0f, 0x1d], "The same note is spent twice in this proof."),
    ([0x09, 0xbd, 0xe3, 0x39], "The pool's verifier rejected the proof."),
    ([0x46, 0x34, 0x95, 0x84], "A fee recipient is named with no fee."),
    ([0x3b, 0x5c, 0xfa, 0x0a], "A withdrawal needs a recipient address."),
    ([0x22, 0x7c, 0x2d, 0x08], "A private transfer cannot name a recipient."),
    ([0xc6, 0x42, 0x00, 0xe9], "The amount is out of range."),
    ([0x0c, 0x82, 0x49, 0xad], "The fee is out of range."),
    ([0x05, 0xa6, 0x4a, 0x81], "Money enters the pool by deposit only."),
    ([0xda, 0xca, 0xc2, 0xdd], "A value in the proof is not a field element."),
    ([0xfb, 0xeb, 0x57, 0xa8], "An address in the proof is not a valid address."),
    ([0xf5, 0xe6, 0x84, 0x62], "The number of sealed notes does not match the outputs."),
    ([0x6d, 0xe3, 0x35, 0xd6], "The public inputs are not a whole number of intents."),
    ([0x05, 0xb9, 0x43, 0x33], "Only the pool's settler may settle right now."),
    ([0x64, 0xe3, 0xef, 0xfb], "This pool is wound down."),
    ([0x5a, 0x65, 0xd1, 0x88], "The pool has paused deposits."),
    ([0x2c, 0x52, 0x11, 0xc6], "The amount is zero, too large, or not whole units."),
    ([0xd8, 0xcc, 0x18, 0xa8], "That EVM address is not on the beta list."),
    ([0xe8, 0x98, 0xdd, 0x7f], "That EVM address has reached its beta limit."),
    ([0x46, 0x03, 0x6b, 0x94], "The pool has reached its beta limit."),
    ([0x43, 0xce, 0xcb, 0xa2], "The beta is paused."),
    ([0xc9, 0x7d, 0x95, 0xcf], "The pool does not know this asset."),
    ([0x96, 0x9b, 0xf7, 0x28], "There is nothing to claim."),
    (
        [0x08, 0x9b, 0xcb, 0xc7],
        "This hand-off's not-before time is off the pool's ten-minute grid.",
    ),
    ([0xe4, 0x65, 0x62, 0x36], "Too early: the pool does not take this hand-off yet."),
    ([0x58, 0x2e, 0xfa, 0x87], "Too many intents in one settlement."),
    ([0x1b, 0xef, 0xca, 0xd1], "The intents in one settlement name different clearing prices."),
    ([0x77, 0xf0, 0xf8, 0x62], "The ETH sent with the call is not what it should be."),
    ([0x32, 0x96, 0x3c, 0x23], "The token moved a different amount than asked."),
    ([0xa2, 0x93, 0xbc, 0x01], "The proof's soundness is below the pool's floor."),
    ([0x37, 0xc8, 0xa8, 0x3c], "The clearing price is out of range."),
    ([0xee, 0x48, 0x42, 0x2e], "The fee is below the policy's protocol part."),
    ([0x6e, 0x1c, 0x06, 0x0b], "The fee's network part is not a rung of the policy's ladder."),
    ([0x0e, 0x85, 0x75, 0x71], "A network part is paid with no submitter to receive it."),
    ([0xb9, 0x1a, 0xa4, 0x3d], "The amount is not a standard size."),
    ([0xd7, 0x5a, 0x7f, 0xb8], "The policy has paused withdrawals of this asset."),
    ([0x7c, 0x8e, 0x60, 0x6e], "The policy has no fee schedule for this asset."),
    ([0x12, 0xe8, 0xeb, 0xa2], "The policy has no size range for this asset."),
];

pub fn explain(revert: &[u8]) -> String {
    let Some(selector) = revert.get(..4).and_then(|s| <[u8; 4]>::try_from(s).ok()) else {
        return "The pool refused the call without saying why.".into();
    };
    SENTENCES.iter().find(|(s, _)| *s == selector).map_or_else(
        || format!("The pool refused the call (0x{}).", hex(&selector)),
        |(_, sentence)| (*sentence).into(),
    )
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
