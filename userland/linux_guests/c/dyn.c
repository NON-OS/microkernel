/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

/*
 * Tier 2: a program the real musl loader brings up, relocating it against a
 * proved libprobe.so. It then asks for a copy of that library with one byte
 * flipped, carrying the good copy's proofs, which the loader must fail to map.
 * Exit 0: linked and refused. 2: the tampered copy loaded. 3: a wrong answer.
 */
#include <dlfcn.h>
#include <stdio.h>

int nonos_probe_value(void);

int main(void) {
    int v = nonos_probe_value();
    printf("[GUEST] dyn: libprobe.so answered %#x\n", v);
    if (v != 0x4e4f) {
        return 3;
    }
    void *bad = dlopen("/lib/libprobe_bad.so", RTLD_NOW);
    if (bad != NULL) {
        printf("[GUEST] dyn ESCAPED: the tampered library loaded\n");
        return 2;
    }
    printf("[GUEST] dyn refused the tampered library: %s\n", dlerror());
    return 0;
}
