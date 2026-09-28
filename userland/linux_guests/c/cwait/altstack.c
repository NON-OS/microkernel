#include "cwait.h"

static long altstack(const stack_t *set, stack_t *old) {
    return syscall(SYS_sigaltstack, set, old) == -1 ? -errno : 0;
}

static char alt[16384] __attribute__((aligned(16)));
static volatile uintptr_t alt_here, alt_saved_rsp, alt_uc_sp;
static volatile long alt_inside_flags = -1, alt_inside_set, alt_uc_flags = -1;

static int on_alt(uintptr_t p) {
    return p > (uintptr_t)alt && p <= (uintptr_t)alt + sizeof alt;
}

static void on_usr2(int sig, siginfo_t *info, void *ctx) {
    (void)sig;
    (void)info;
    char here;
    alt_here = (uintptr_t)&here;
    stack_t now, other = {.ss_sp = alt, .ss_size = sizeof alt, .ss_flags = 0};
    alt_inside_flags = altstack(0, &now) == 0 ? now.ss_flags : -1;
    alt_inside_set = altstack(&other, 0);
    ucontext_t *uc = ctx;
    alt_uc_flags = uc->uc_stack.ss_flags;
    alt_uc_sp = (uintptr_t)uc->uc_stack.ss_sp;
    alt_saved_rsp = (uintptr_t)uc->uc_mcontext.gregs[REG_RSP];
}

int alternate_stack(void) {
    stack_t got, small = {.ss_sp = alt, .ss_size = 1024, .ss_flags = 0};
    stack_t bad = {.ss_sp = alt, .ss_size = sizeof alt, .ss_flags = 4};
    stack_t set = {.ss_sp = alt, .ss_size = sizeof alt, .ss_flags = 0};
    stack_t off = {.ss_flags = SS_DISABLE};
    long none = altstack(0, &got) == 0 ? got.ss_flags : -1;
    long nomem = altstack(&small, 0), inval = altstack(&bad, 0);
    if (none != SS_DISABLE || nomem != -ENOMEM || inval != -EINVAL) {
        return fail("sigaltstack before one is set", none * 100 - nomem, inval);
    }
    long rc = altstack(&set, 0);
    long flags = altstack(0, &got) == 0 ? got.ss_flags : -1;
    if (rc != 0 || flags != 0 || got.ss_sp != alt || got.ss_size != sizeof alt) {
        return fail("sigaltstack set and read back", rc, flags);
    }
    struct sigaction sa;
    memset(&sa, 0, sizeof sa);
    sa.sa_sigaction = on_usr2;
    sa.sa_flags = SA_SIGINFO | SA_ONSTACK;
    sigaction(SIGUSR2, &sa, 0);
    syscall(SYS_tgkill, getpid(), gettid(), SIGUSR2);
    getpid();
    long off_rc = altstack(&off, 0);
    long after = altstack(0, &got) == 0 ? got.ss_flags : -1;
    if (!on_alt(alt_here) || on_alt(alt_saved_rsp) || alt_uc_sp != (uintptr_t)alt) {
        return fail("SA_ONSTACK handler on the alternate stack", on_alt(alt_here),
                    on_alt(alt_saved_rsp));
    }
    if (alt_inside_flags != SS_ONSTACK || alt_inside_set != -EPERM || alt_uc_flags != 0) {
        return fail("sigaltstack inside the handler", alt_inside_flags, alt_inside_set);
    }
    if (off_rc != 0 || after != SS_DISABLE) {
        return fail("sigaltstack disabled", off_rc, after);
    }
    ok("altstack",
       "ENOMEM, EINVAL, set, SA_ONSTACK handler on it with SS_ONSTACK and EPERM inside, "
       "disabled; flags inside",
       alt_inside_flags);
    return 0;
}
