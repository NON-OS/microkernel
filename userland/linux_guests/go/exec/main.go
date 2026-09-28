/*
 * Starting programs from Go. syscall.ForkExec is Go's own clone(CLONE_VFORK|
 * CLONE_VM|SIGCHLD) and execve, with the child's exec error sent back through
 * a pipe; Wait4 then reads each child's status. The children are the pthreads
 * guest, a program that ends with status 42, and a path that does not exist.
 * Then os/exec does the same through cmd.Output, which also needs the
 * runtime's poller (eventfd, epoll) for its pipes; those parts run last.
 * Stdin, and for Run the output too, is this program's own: a nil one makes
 * os/exec open /dev/null, which the personality does not serve.
 */
package main

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"os/exec"
	"strings"
	"syscall"
)

var failed, parts int

func part(ok bool, what string) {
	mark := "ok"
	if !ok {
		mark = "FAIL"
		failed++
	}
	parts++
	fmt.Printf("[GO] goexec %s: %s\n", mark, what)
}

func own(c *exec.Cmd) *exec.Cmd {
	c.Stdin, c.Stdout, c.Stderr = os.Stdin, os.Stdout, os.Stderr
	return c
}

func main() {
	forkExecParts()

	cmd := exec.Command("/bin/cthreads")
	cmd.Stdin = os.Stdin
	out, err := cmd.Output()
	line := strings.TrimSpace(string(out))
	code := -1
	if cmd.ProcessState != nil {
		code = cmd.ProcessState.ExitCode()
	}
	part(err == nil && code == 0 && strings.Contains(line, "cthreads PASS"),
		fmt.Sprintf("os/exec cthreads Output: status %d, err %v, output %q", code, err, line))

	err = own(exec.Command("/bin/leaderexit")).Run()
	var ee *exec.ExitError
	part(errors.As(err, &ee) && ee.ExitCode() == 42,
		fmt.Sprintf("os/exec leaderexit's status came back: %v", err))

	err = own(exec.Command("/bin/no-such-program")).Run()
	var pe *fs.PathError
	part(errors.As(err, &pe) && pe.Op == "fork/exec" && errors.Is(pe.Err, syscall.ENOENT),
		fmt.Sprintf("os/exec a missing program: %v", err))

	verdict := "PASS"
	if failed != 0 {
		verdict = "FAIL"
	}
	fmt.Printf("[GO] goexec %s: %d of %d parts held\n", verdict, parts-failed, parts)
	if failed != 0 {
		os.Exit(1)
	}
}
