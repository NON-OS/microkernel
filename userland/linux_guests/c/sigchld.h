/*
 * What the sigchld guest's files share: the part line every check prints,
 * a child that exits after a delay, and the checks each file holds.
 */
#include <sys/types.h>

void part(int ok, const char *what);
pid_t child_exiting(int code, unsigned delay_ms);
void waits(void);
void spawns(void);
