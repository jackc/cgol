#include "stdafx.h"

#include "cgol.h"


world::world(int width, int height) {
	this->width = width;
	this->height = height;
	cells.resize(width*height);
	scratch.resize(width*height);
}

int world::getWidth() {
	return width;
}

int world::getHeight() {
	return height;
}

// idxFromCoord takes x and y coordinates and returns the index in cells.
// Coordinates wrap the boundaries of the world. e.g. Given world with a
// width of 10, then an x coordinate of -1 should be equal to 9.
int world::idxFromCoord(int x, int y) {
	x = x % width;
	if (x < 0) {
		x += width;
	}
	y = y % height;
	if (y < 0) {
		y += height;
	}

	return y*width + x;
}

void world::set(int x, int y, bool val) {
	cells[idxFromCoord(x, y)] = val;
}

bool world::get(int x, int y) {
	return cells[idxFromCoord(x, y)];
}

int world::countNeighbors(int x, int y) {
	int liveNeighborCount = 0;
	liveNeighborCount += get(x - 1, y - 1);
	liveNeighborCount += get(x, y - 1);
	liveNeighborCount += get(x + 1, y - 1);

	liveNeighborCount += get(x - 1, y);
	liveNeighborCount += get(x + 1, y);

	liveNeighborCount += get(x - 1, y + 1);
	liveNeighborCount += get(x, y + 1);
	liveNeighborCount += get(x + 1, y + 1);

	return liveNeighborCount;
}

void world::step() {
	for (int y = 0; y < getHeight(); y++) {
		for (int x = 0; x < getWidth(); x++) {
			bool newValue;
			auto neighborCount = countNeighbors(x, y);
			if (get(x, y)) {
				newValue = neighborCount == 2 || neighborCount == 3;
			}
			else {
				newValue = neighborCount == 3;
			}
			scratch[idxFromCoord(x, y)] = newValue;
		}
	}

	cells.swap(scratch);
}


