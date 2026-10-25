use super::roles::Roles;

/* Ink and Paper from nonos.software/brand; Paper's accent is #007C82, as the
 * brand's #007C85 is 4.49:1 on Paper (theme/schemes/nonos.rs). */
pub const INK: Roles = Roles {
    ground: 0xFF0A_0B0D,
    surface: 0xFF10_1215,
    raised: 0xFF16_181B,
    text: 0xFFF2_F3F7,
    text2: 0xFFB3_B4B7,
    text3: 0xFF8A_8B8F,
    rule: 0xFF26_282B,
    edge: 0xFF62_646A,
    accent: 0xFF66_FFFF,
    on_accent: 0xFF0A_0B0D,
    select: 0xFF12_302F,
    ok: 0xFF5E_E0A0,
    warn: 0xFFFF_B547,
    bad: 0xFFFF_6B6F,
    on_bad: 0xFF0A_0B0D,
};

pub const PAPER: Roles = Roles {
    ground: 0xFFF2_F3F7,
    surface: 0xFFFF_FFFF,
    raised: 0xFFE8_E9EE,
    text: 0xFF0A_0B0D,
    text2: 0xFF45_474D,
    text3: 0xFF63_656B,
    rule: 0xFFD5_D7DD,
    edge: 0xFF7E_8188,
    accent: 0xFF00_7C82,
    on_accent: 0xFFFF_FFFF,
    select: 0xFFDC_EFF0,
    ok: 0xFF17_703F,
    warn: 0xFF8A_5A00,
    bad: 0xFFC4_262E,
    on_bad: 0xFFFF_FFFF,
};

/* Every text role at 7:1 or more on every ground, the accent the existing
 * high-contrast scheme's yellow. */
pub const HIGH_CONTRAST: Roles = Roles {
    ground: 0xFF00_0000,
    surface: 0xFF00_0000,
    raised: 0xFF14_1414,
    text: 0xFFFF_FFFF,
    text2: 0xFFFF_FFFF,
    text3: 0xFFE6_E6E6,
    rule: 0xFFFF_FFFF,
    edge: 0xFFFF_FFFF,
    accent: 0xFFFF_FF00,
    on_accent: 0xFF00_0000,
    select: 0xFF33_3300,
    ok: 0xFF66_FF99,
    warn: 0xFFFF_D24D,
    bad: 0xFFFF_8A8A,
    on_bad: 0xFF00_0000,
};
