/* What the memcalls files share. */
#ifndef MEMCALLS_H
#define MEMCALLS_H

#include "memproof.h"

#define NOREPLACE 0x100000

/* The page a part run in a child touches. */
extern volatile char *mc_p;

void check(const char *name, int ok, long got);
/* A part run in a forked child that must die of SIGSEGV. */
void faults(const char *name, void (*fn)(void));
/* -errno of a call that returned -1, or its value. */
long rc(long v);
char *anon(long len, int prot);
void mc_write(void);
void mc_read(void);

void placement(void);
void breaks(void);
void alignment(void);
void remaps(void);
void provenance(void);
void locks(void);
void syncs(void);
void cores(void);

#endif
