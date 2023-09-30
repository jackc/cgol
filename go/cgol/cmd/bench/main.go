package main

import (
	"flag"
	"fmt"
	"math/rand"
	"os"
	"time"

	"github.com/jackc/cgol/go/cgol"
)

var options struct {
	width     int
	height    int
	cellCount int
	stepCount int
	seed      int
}

func main() {
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "usage:  %s [options]\n", os.Args[0])
		flag.PrintDefaults()
	}

	flag.IntVar(&options.width, "width", 100, "width of world")
	flag.IntVar(&options.height, "height", 100, "height of world")
	flag.IntVar(&options.cellCount, "livecount", 100, "live cells to start")
	flag.IntVar(&options.stepCount, "stepcount", 1_000, "number of steps to run")
	flag.IntVar(&options.seed, "seed", -1, "seed")
	flag.Parse()

	w := cgol.NewWorld(options.width, options.height)

	rng := rand.New(rand.NewSource(int64(options.seed)))

	for i := 0; i < options.cellCount; i++ {
		w.Set(rng.Intn(w.Width()), rng.Intn(w.Height()), true)
	}

	startTime := time.Now()
	for i := 0; i < options.stepCount; i++ {
		w = w.Step()
	}
	endTime := time.Now()

	duration := endTime.Sub(startTime)
	fmt.Println("Go:", duration.Seconds())
}
