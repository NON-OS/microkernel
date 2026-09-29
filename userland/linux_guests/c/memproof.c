/*
 * The memory proofs as one program, so the test store carries one binary and
 * one set of proofs for all of them instead of one per proof: the store has a
 * fixed load budget and each proof set is most of a guest's size there. The
 * first argument names the proof; each is its own file, built with its main
 * renamed, and runs exactly as it would as a program of its own.
 */
#include <stdio.h>
#include <string.h>

int guardpage_main(void);
int protnone_main(void);
int protfork_main(void);
int touchfork_main(void);
int memcalls_main(void);

static const struct {
    const char *name;
    int (*run)(void);
} proofs[] = {
    { "guardpage", guardpage_main },
    { "protnone", protnone_main },
    { "protfork", protfork_main },
    { "touchfork", touchfork_main },
    { "memcalls", memcalls_main },
};

int main(int argc, char **argv) {
    for (unsigned i = 0; argc > 1 && i < sizeof proofs / sizeof proofs[0]; i++) {
        if (strcmp(argv[1], proofs[i].name) == 0) {
            return proofs[i].run();
        }
    }
    fputs("[C] memproof FAIL: name a proof: guardpage protnone protfork touchfork memcalls\n",
          stdout);
    return 2;
}
