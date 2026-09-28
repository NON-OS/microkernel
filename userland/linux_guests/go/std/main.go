// The start of a Go standard-library test inside the guest: go test runs each
// test binary from its package's directory, where testdata/ sits, and the boot
// guest always starts at /. So this changes to the directory named first and
// then becomes the program named after it, with the rest as its arguments.
// NAME=value words between the two are added to the environment, as env(1)
// adds them, so a run can set GODEBUG or GOTRACEBACK:
//
//	/bin/gostd /usr/local/go/src/time GODEBUG=schedtrace=1000 /bin/gstime -test.v
//
// A refused chdir or execve is printed with its errno and ends with status
// 127, the shell's status for a program that could not be run.
package main

import (
	"fmt"
	"os"
	"strings"
	"syscall"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Fprintln(os.Stderr, "[GOSTD] usage: gostd <dir> <program> [args...]")
		os.Exit(127)
	}
	dir, rest, env := os.Args[1], os.Args[2:], os.Environ()
	for len(rest) > 1 && !strings.HasPrefix(rest[0], "/") && strings.Contains(rest[0], "=") {
		env, rest = append(env, rest[0]), rest[1:]
	}
	if err := syscall.Chdir(dir); err != nil {
		fmt.Fprintf(os.Stderr, "[GOSTD] chdir %s: %v\n", dir, err)
		os.Exit(127)
	}
	prog, argv := rest[0], rest
	err := syscall.Exec(prog, argv, env)
	fmt.Fprintf(os.Stderr, "[GOSTD] execve %s: %v\n", prog, err)
	os.Exit(127)
}
