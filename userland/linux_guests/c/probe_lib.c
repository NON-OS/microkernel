/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

/*
 * The shared library the dynamic guest links against. Its answer is a value
 * the program checks, so a loader that mapped the wrong bytes shows up as a
 * wrong answer rather than a silent success.
 */
int nonos_probe_value(void) { return 0x4e4f; }
