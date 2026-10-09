/*
 * The syscall.ForkExec half of goexec: Go's own clone(CLONE_VFORK|CLONE_VM|
 * SIGCHLD) and execve, with the child's exec error sent back through a pipe,
 * then Wait4 for each child's status. Nothing here needs the runtime's poller.
 */
package main

import (
	"errors"
	"fmt"
	"syscall"
)

/* run starts path with syscall.ForkExec and waits for it with Wait4. */
func run(path string) (syscall.WaitStatus, error) {
	attr := &syscall.ProcAttr{Files: []uintptr{0, 1, 2}}
	pid, err := syscall.ForkExec(path, []string{path}, attr)
	if err != nil {
		return 0, err
	}
	var ws syscall.WaitStatus
	got, err := syscall.Wait4(pid, &ws, 0, nil)
	if err == nil && got != pid {
		err = fmt.Errorf("wait4 answered %d for child %d", got, pid)
	}
	return ws, err
}

func forkExecParts() {
	ws, err := run("/bin/cthreads")
	part(err == nil && ws.Exited() && ws.ExitStatus() == 0,
		fmt.Sprintf("ForkExec cthreads: status %d, err %v", ws.ExitStatus(), err))
	ws, err = run("/bin/leaderexit")
	part(err == nil && ws.Exited() && ws.ExitStatus() == 42,
		fmt.Sprintf("ForkExec leaderexit: status %d, err %v", ws.ExitStatus(), err))
	_, err = run("/bin/no-such-program")
	part(errors.Is(err, syscall.ENOENT), fmt.Sprintf("ForkExec a missing program: %v", err))
	fmt.Printf("[GO] goexec: ForkExec parts done, %d of %d held\n", parts-failed, parts)
}
