/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/*
 * One bit per app setup may turn off; a set bit is off. The same bits as
 * nonos_policy_proto::apps, which setup and the desktop shell read.
 */

pub(crate) const BROWSER: u32 = 1 << 0;
pub(crate) const WALLET: u32 = 1 << 1;
/* The store window. The package service behind it is not covered. */
pub(crate) const STORE: u32 = 1 << 2;
pub(crate) const FILES: u32 = 1 << 3;
pub(crate) const EDITOR: u32 = 1 << 4;
pub(crate) const CALCULATOR: u32 = 1 << 5;
/* The music player, the video player and the image viewer together. */
pub(crate) const MEDIA: u32 = 1 << 6;
/* The Linux personality, and so Qwen and every Linux package it runs. */
pub(crate) const LINUX: u32 = 1 << 7;
