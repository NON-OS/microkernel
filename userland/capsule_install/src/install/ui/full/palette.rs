/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * First-boot setup's colours (capsule_setup_wizard render/theme.rs), so
 * the screen does not change look when setup hands over to the installer.
 */

pub const BACKDROP: u32 = 0xFF0B_0F16;
pub(super) const CARD_BG: u32 = 0xFF16_1E2A;
pub(super) const ACCENT: u32 = 0xFF4F_D1C5;
pub(super) const FG: u32 = 0xFFFF_FFFF;
pub(super) const HINT: u32 = 0xFF8A_A0B8;
pub(super) const RULE: u32 = 0xFF24_3246;
pub(super) const GRAD_TOP: u32 = 0xFF0D_2B29;
pub(super) const GRAD_BOT: u32 = BACKDROP;
pub(super) const DOT_DONE: u32 = ACCENT;
pub(super) const DOT_CUR: u32 = FG;
pub(super) const DOT_TODO: u32 = 0xFF32_4054;
