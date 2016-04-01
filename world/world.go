package world

type World struct {
	cells  []bool
	width  int
	height int
}

func New(width, height int) *World {
	cells := make([]bool, width*height)
	return &World{cells: cells, width: width, height: height}
}

func (w *World) Width() int {
	return w.width
}

func (w *World) Height() int {
	return w.height
}

func (w *World) Step() *World {
	w2 := New(w.width, w.height)

	for y := 0; y < w.Height(); y++ {
		for x := 0; x < w.Width(); x++ {
			var newValue bool
			neighborCount := w.countNeighbors(x, y)
			if w.Get(x, y) {
				newValue = neighborCount == 2 || neighborCount == 3
			} else {
				newValue = neighborCount == 3
			}
			w2.Set(x, y, newValue)
		}
	}

	return w2
}

func (w *World) Set(x, y int, val bool) {
	idx := w.idxFromCoord(x, y)
	w.cells[idx] = val
}

func (w *World) Get(x, y int) bool {
	idx := w.idxFromCoord(x, y)
	return w.cells[idx]
}

func (w *World) countNeighbors(x, y int) int {
	neighborValues := [8]bool{
		w.Get(x-1, y-1),
		w.Get(x, y-1),
		w.Get(x+1, y-1),

		w.Get(x-1, y),
		w.Get(x+1, y),

		w.Get(x-1, y+1),
		w.Get(x, y+1),
		w.Get(x+1, y+1),
	}

	liveNeighborCount := 0
	for _, v := range neighborValues {
		if v {
			liveNeighborCount++
		}
	}

	return liveNeighborCount
}

// idxFromCoord takes x and y coordinates and returns the index in w.cells.
// Coordinates wrap the boundaries of the world. e.g. Given World with a
// width of 10, then an x coordinate of -1 should be equal to 9.
func (w *World) idxFromCoord(x, y int) int {
	x = x % w.width
	if x < 0 {
		x += w.width
	}
	y = y % w.height
	if y < 0 {
		y += w.height
	}

	return y*w.width + x
}
