/* The NØNOS colour roles (HANDOFF/ui/identity.md, Colour): every colour a
 * NØNOS screen is drawn in, by what it is for. The theme engine's five seeds
 * come from these; the rest are the roles the engine does not derive. Each
 * ratio identity.md gives is asserted from these values in theme_proofs, so a
 * value that stops meeting its ratio fails there, not on a screen. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Roles {
    pub ground: u32,
    pub surface: u32,
    pub raised: u32,
    pub text: u32,
    /* Secondary text; also the secondary line of a selected row. */
    pub text2: u32,
    /* Metadata and placeholders, never below 4.5:1, never on `select`. */
    pub text3: u32,
    /* A dividing rule: decoration, under 3:1 by design. */
    pub rule: u32,
    /* The outline of an input, switch or button: 3:1 on every ground. */
    pub edge: u32,
    pub accent: u32,
    pub on_accent: u32,
    /* A selected row's fill, always beside a 2 px accent edge. */
    pub select: u32,
    /* Proposed, owner to confirm: the brand names no success colour. */
    pub ok: u32,
    pub warn: u32,
    pub bad: u32,
    pub on_bad: u32,
}

pub use super::roles_set::{HIGH_CONTRAST, INK, PAPER};
