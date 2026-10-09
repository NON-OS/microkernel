/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

/* One bit per app setup may turn off; a set bit is off. */

pub const BROWSER: u8 = 1 << 0;
pub const WALLET: u8 = 1 << 1;
/* The store window. The package service behind it is not covered. */
pub const STORE: u8 = 1 << 2;
pub const FILES: u8 = 1 << 3;
pub const EDITOR: u8 = 1 << 4;
pub const CALCULATOR: u8 = 1 << 5;
/* The music player, the video player and the image viewer together. */
pub const MEDIA: u8 = 1 << 6;
/* The Linux personality, and so Qwen and every Linux package it runs. */
pub const LINUX: u8 = 1 << 7;

/* Every app on. */
pub const NONE_OFF: u8 = 0;
