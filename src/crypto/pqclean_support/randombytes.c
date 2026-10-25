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

/* Fails, rather than handing over weaker bytes, when the seeded RNG
 * cannot answer; the Rust side counts the failure, since the callers of
 * this function never look at what it returns. */
extern int nonos_randombytes(uint8_t *buf, size_t n);

int randombytes(uint8_t *output, size_t n) {
    return nonos_randombytes(output, n);
}
