// Concurrency without the netpoller: goroutines, channels and a wait group,
// which lean on thread creation (clone), futex-based parking and the
// scheduler. The sum is known, so a wrong answer is a broken runtime.
package main

import (
	"fmt"
	"os"
	"runtime"
	"sync"
)

func main() {
	const workers, each = 8, 10000
	runtime.GOMAXPROCS(4)
	out := make(chan int, workers)
	var wg sync.WaitGroup
	for w := 0; w < workers; w++ {
		wg.Add(1)
		go func(base int) {
			defer wg.Done()
			sum := 0
			for i := 0; i < each; i++ {
				sum += base + i
			}
			out <- sum
		}(w * each)
	}
	go func() { wg.Wait(); close(out) }()
	total := 0
	for s := range out {
		total += s
	}
	want := (workers*each - 1) * (workers * each) / 2
	if total != want {
		fmt.Printf("[GO] conc FAIL: total=%d want=%d\n", total, want)
		os.Exit(1)
	}
	fmt.Printf("[GO] conc PASS: %d goroutines summed %d\n", workers, total)
	os.Exit(0)
}
