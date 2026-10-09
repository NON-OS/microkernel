/*
 * A 12 MB program that execs itself once. The personality reads a program
 * whole before it runs it, so this second read of the same 12 MB happens
 * while the first program's bytes could still be held: it passes only if
 * the personality let them go once the first one was running.
 *
 *   /bin/execbig          prints its first line, then execs /bin/execbig again
 *   /bin/execbig second   prints its second line and ends with status 0
 */
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

/* Initialised, so all 12 MiB are in the file, not left for the loader. */
static volatile char blob[12 << 20] = { 1 };

int main(int argc, char **argv)
{
	char *again[] = { "/bin/execbig", "second", NULL };

	if (argc > 1 && strcmp(argv[1], "second") == 0) {
		printf("[EXECBIG] second image running, first byte %d\n", blob[0]);
		return 0;
	}
	printf("[EXECBIG] first image running, %zu bytes of data\n", sizeof(blob));
	fflush(stdout);
	execve(again[0], again, NULL);
	printf("[EXECBIG] execve refused: %s\n", strerror(errno));
	return 1;
}
