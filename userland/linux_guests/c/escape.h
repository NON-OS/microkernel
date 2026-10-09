/* What the escape attack files share. Each part passes when NONOS refuses. */
#ifndef ESCAPE_H
#define ESCAPE_H

#include "memproof.h"

/* Record an attack: ok means the machine held (refused it). */
void held(const char *name, int ok, long got);
/* -errno of a call (escape) that returned -1, or its value. */
long erc(long v);

void reach_kernel(void);
void wrap_span(void);
void wx_map(void);
void exec_escalate(void);
void forged_call(void);
void past_end(void);

#endif
