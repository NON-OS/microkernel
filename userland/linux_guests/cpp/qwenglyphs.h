/* JetBrains Mono (OFL) at 10x22 cells, ASCII 32 to 126, 8-bit alpha. */
#pragma once
enum { CELL_W = 10, CELL_H = 22, FIRST = 32, LAST = 126 };
static const char *const GLYPH_HEX[LAST - FIRST + 1] = {
#include "qwenglyphs_32.h"
#include "qwenglyphs_64.h"
#include "qwenglyphs_96.h"
};

static inline unsigned hex_nibble(char c) { return c <= '9' ? c - '0' : c - 'a' + 10; }

/* Alpha of row j, column k of glyph g (0 is FIRST). */
static inline unsigned glyph_alpha(int g, int j, int k) {
    const char *p = GLYPH_HEX[g] + 2 * (j * CELL_W + k);
    return hex_nibble(p[0]) << 4 | hex_nibble(p[1]);
}
