/*
 * What every memory proof shares: printing a line, running a part in a forked
 * child, reading how the child ended, and counting parts for the last line.
 */
#ifndef MEMPROOF_H
#define MEMPROOF_H

#define PG 4096

void say(const char *s);

/*
 * A SIGSEGV death. The personality reports a signal death as exit status
 * 128+signo, which counts as the same SIGSEGV; the raw status is printed.
 */
int segv(int st);

/* Count one part and print its line: "[C] <proof> <name>: ok|FAIL (<detail>)". */
void part_line(const char *proof, const char *name, int ok, const char *detail);

/* Run `fn` in a forked child that must, or must not, die of SIGSEGV. */
void part(const char *proof, const char *name, void (*fn)(void), int must_fault);

/* Print the PASS or FAIL line and give the exit code. */
int finish(const char *proof);

#endif
