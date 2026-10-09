// Go's network poller, which any timer or pollable file starts: epoll_create1,
// an eventfd, and epoll_pwait with a timeout. Three parts, each printed as it
// passes, so a hang names the part it hung in:
//
//   - a 50 ms sleep ends on time, which needs epoll_pwait to wait its timeout;
//   - a 10 ms timer set while the poller already waits on a 3 s one ends in
//     well under 3 s, which needs a write to Go's eventfd from another thread
//     to end that wait;
//   - a pipe read through the poller gets every write and then end of file,
//     which needs O_NONBLOCK kept, edge-triggered readiness, and a hang-up
//     once the write end is closed.
package main

import (
	"fmt"
	"io"
	"os"
	"syscall"
	"time"
)

func fail(format string, args ...any) {
	fmt.Printf("[GO] poll FAIL: "+format+"\n", args...)
	os.Exit(1)
}

func main() {
	start := time.Now()
	time.Sleep(50 * time.Millisecond)
	slept := time.Since(start).Milliseconds()
	if slept < 50 || slept > 1000 {
		fail("a 50ms sleep took %dms", slept)
	}
	fmt.Printf("[GO] poll sleep ok: 50ms slept %dms\n", slept)

	long := make(chan struct{})
	short := make(chan int64, 1)
	go func() { time.Sleep(3 * time.Second); close(long) }()
	go func() {
		// A raw nanosleep keeps this goroutine away from the poller while the
		// long sleep puts the poller into its 3 s wait.
		ts := syscall.Timespec{Nsec: 200 * 1000 * 1000}
		syscall.Nanosleep(&ts, nil)
		t0 := time.Now()
		time.Sleep(10 * time.Millisecond)
		short <- time.Since(t0).Milliseconds()
	}()
	quick := <-short
	<-long
	if quick > 1000 {
		fail("a 10ms timer set under a 3s one took %dms", quick)
	}
	fmt.Printf("[GO] poll wake ok: a 10ms timer set under a 3s one took %dms\n", quick)

	r, w, err := os.Pipe()
	if err != nil {
		fail("pipe: %v", err)
	}
	go func() {
		for i := 0; i < 3; i++ {
			time.Sleep(100 * time.Millisecond)
			fmt.Fprintf(w, "m%d;", i)
		}
		w.Close()
	}()
	t0 := time.Now()
	got, err := io.ReadAll(r)
	took := time.Since(t0).Milliseconds()
	if err != nil || string(got) != "m0;m1;m2;" {
		fail("pipe read %q, err %v", got, err)
	}
	fmt.Printf("[GO] poll pipe ok: read %q to end of file in %dms\n", got, took)
	fmt.Printf("[GO] poll PASS: 3 parts, %dms in all\n", time.Since(start).Milliseconds())
	os.Exit(0)
}
