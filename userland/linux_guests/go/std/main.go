// The start of a Go standard-library test inside the guest: go test runs each
// test binary from its package's directory, where testdata/ sits, and the boot
// guest always starts at /. So this changes to the directory named first and
// then becomes the program named second, with the rest as its arguments and
// the environment unchanged:
//
//	/bin/gostd /usr/local/go/src/time /bin/gstime -test.v -test.short
//
// A refused chdir or execve is printed with its errno and ends with status
// 127, the shell's status for a program that could not be run.
package main

import (
	"fmt"
	"os"
	"syscall"
)

func main() {
	if len(os.Args) < 3 {
		fmt.Fprintln(os.Stderr, "[GOSTD] usage: gostd <dir> <program> [args...]")
		os.Exit(127)
	}
	dir, prog := os.Args[1], os.Args[2]
	if err := syscall.Chdir(dir); err != nil {
		fmt.Fprintf(os.Stderr, "[GOSTD] chdir %s: %v\n", dir, err)
		os.Exit(127)
	}
	argv := append([]string{prog}, os.Args[3:]...)
	err := syscall.Exec(prog, argv, os.Environ())
	fmt.Fprintf(os.Stderr, "[GOSTD] execve %s: %v\n", prog, err)
	os.Exit(127)
}
