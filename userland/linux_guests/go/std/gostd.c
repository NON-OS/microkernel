/*
 * The start of a Go standard-library test inside the guest. go test runs each
 * test binary from its package's directory, beside testdata/, and the boot
 * guest always starts at /. So this changes to the directory named first and
 * becomes the program named after it, with the rest as its arguments.
 * NAME=value words between the two are added to the environment, as env(1)
 * adds them, so a run can set GODEBUG or GOTRACEBACK:
 *
 *   /bin/gostd /usr/local/go/src/time GODEBUG=schedtrace=1000 /bin/gstime -test.v
 *
 * It is C, not Go: a Go runtime here would take signals of its own before the
 * test starts, and the test's own settings could not reach it. A refused chdir
 * or execve is printed with its errno and ends with status 127, the shell's
 * status for a program that could not be run.
 */
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

extern char **environ;

int main(int argc, char **argv)
{
	int i = 2;

	if (argc < 3) {
		fprintf(stderr, "[GOSTD] usage: gostd <dir> [NAME=value...] <program> [args...]\n");
		return 127;
	}
	for (; i < argc - 1 && argv[i][0] != '/' && strchr(argv[i], '='); i++)
		putenv(argv[i]);
	if (chdir(argv[1]) != 0) {
		fprintf(stderr, "[GOSTD] chdir %s: %s\n", argv[1], strerror(errno));
		return 127;
	}
	execve(argv[i], argv + i, environ);
	fprintf(stderr, "[GOSTD] execve %s: %s\n", argv[i], strerror(errno));
	return 127;
}
