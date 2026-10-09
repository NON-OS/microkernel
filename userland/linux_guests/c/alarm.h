/*
 * What the alarm guest's two halves share: its SIGALRM and SIGUSR1 counts,
 * what the SIGUSR1 handler read from its ucontext, and the part line every
 * check prints.
 */
#include <signal.h>
#include <time.h>

extern volatile sig_atomic_t alarms, usr1, usr1_rip_in_text, usr1_saved_blocked;

void part(int ok, const char *what, long n);
long ms_since(struct timespec *t0);
void catch(int sig, void (*fn)(int));
void signal_waits(void);
void posix_timer(void);
void signal_fd(void);
int verdict(void);
