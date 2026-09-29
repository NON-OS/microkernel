/*
 * A Linux guest trying to break out of its own address space through the
 * syscall surface it is given: reach the kernel half, wrap a span, map
 * write-and-execute, add execute to a file it never proved, call a number
 * that is not served, and step past the end of what it holds. Each part
 * passes when the machine refuses; one boot names any that got through.
 */
#include <errno.h>
#include <stdio.h>

#include "escape.h"

void held(const char *name, int ok, long got) {
    char detail[48];
    snprintf(detail, sizeof detail, "got %ld", got);
    part_line("escape", name, ok, detail);
}

long erc(long v) {
    return v == -1 ? -errno : v;
}

int main(void) {
    reach_kernel();
    wrap_span();
    wx_map();
    exec_escalate();
    forged_call();
    past_end();
    return finish("escape");
}
