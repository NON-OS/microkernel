/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * The JS prelude, as one string built from three parts.
 */

/* The window shims frameworks probe for before rendering: timer queues that
 * flush on demand, a quiet console, and enough of navigator/location to pass
 * feature checks. Kept in JS so host and capsule share one definition. */
static const char *PRELUDE =
#include "dom_prelude_1.inc"
#include "dom_prelude_2.inc"
#include "dom_prelude_3.inc"
