#pragma once

#include <vector>

using namespace std;

class world {
	vector<bool> cells;
	vector<bool> scratch;
	int width;
	int height;

	int idxFromCoord(int x, int y);
	int countNeighbors(int x, int y);

public:
	world(int width, int height);
	int getWidth();
	int getHeight();

	void set(int x, int y, bool val);
	bool get(int x, int y);

	void step();
};