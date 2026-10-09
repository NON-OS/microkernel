#include "cwait.h"

static int two_in;
static char two_got[2];
static void *pipe_reader(void *arg) {
    read(two_in, &two_got[(uintptr_t)arg], 1);
    return 0;
}


int two_readers(void) {
    int p[2];
    pipe(p);
    two_in = p[0];
    pthread_t t[2];
    pthread_create(&t[0], 0, pipe_reader, (void *)0);
    pthread_create(&t[1], 0, pipe_reader, (void *)1);
    nap_ms(100);
    write(p[1], "ab", 2);
    pthread_join(t[0], 0);
    pthread_join(t[1], 0);
    close(p[0]);
    close(p[1]);
    int both = (two_got[0] == 'a' && two_got[1] == 'b') || (two_got[0] == 'b' && two_got[1] == 'a');
    if (!both) {
        return fail("two blocked pipe readers", two_got[0], two_got[1]);
    }
    ok("pipe-readers", "two threads blocked reading one pipe, both answered; bytes", 2);
    return 0;
}
