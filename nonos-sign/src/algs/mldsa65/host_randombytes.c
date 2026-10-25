// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#include "randombytes.h"

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

/* The signing code never looks at what randombytes returns, so a failed
 * draw would leave it working from a zeroed buffer. Each failure is noted
 * here; the Rust side takes the note after every call and refuses. */
static int nonos_deterministic_random;
static int nonos_random_failed;
static _Thread_local int nonos_force_failure;

void NONOS_mldsa65_deterministic_random(int enabled) {
    nonos_deterministic_random = enabled;
}

/* For the tests only: make every draw from the system fail, on this
 * thread alone, so other tests running beside it are untouched. */
void NONOS_mldsa65_force_random_failure(int enabled) {
    nonos_force_failure = enabled;
}

/* Whether any draw failed since the last call, clearing the note. */
int NONOS_mldsa65_take_random_failure(void) {
    int failed = nonos_random_failed;
    nonos_random_failed = 0;
    return failed;
}

static int fail(uint8_t *buf, size_t n) {
    memset(buf, 0, n);
    nonos_random_failed = 1;
    return -1;
}

int randombytes(uint8_t *buf, size_t n) {
    if (nonos_deterministic_random) {
        memset(buf, 0, n);
        return 0;
    }
    FILE *f = nonos_force_failure ? NULL : fopen("/dev/urandom", "rb");
    if (!f) {
        return fail(buf, n);
    }
    size_t got = 0;
    while (got < n) {
        size_t r = fread(buf + got, 1, n - got, f);
        if (r == 0) {
            fclose(f);
            return fail(buf, n);
        }
        got += r;
    }
    fclose(f);
    return 0;
}
