// A goroutine that spins without a call can be stopped only by a signal:
// Go's sysmon sends SIGURG to its thread once it has run 10 ms, and the
// handler moves it off the CPU. With one P nothing else runs until then, so
// main sleeping 20 ms and then collecting garbage, which stops the world,
// both need that signal to land on a thread that is running, not parked.
package main

import (
	"fmt"
	"os"
	"runtime"
	"time"
)

var spun uint64

func spin() {
	for {
		spun++
	}
}

func main() {
	runtime.GOMAXPROCS(1)
	start := time.Now()
	go spin()
	time.Sleep(20 * time.Millisecond)
	woke := time.Since(start).Milliseconds()
	runtime.GC()
	collected := time.Since(start).Milliseconds()
	fmt.Printf("[GO] preempt PASS: main ran again after %dms and collected by %dms beside a spinning goroutine\n",
		woke, collected)
	os.Exit(0)
}
