#include "stdafx.h"

#include <iostream>
#include <memory>
#include <random>
#include <chrono>
#include <thread>
#include <tclap/CmdLine.h>

#include "cgol.h"

struct cliOptions {
	int width;
	int height;
	int liveCount;
	int stepTime;
	int seed;
};

std::unique_ptr<cliOptions> parseCLI(int argc, const char * argv[]);
void render(world& w);

int main(int argc, const char * argv[]) {
	std::unique_ptr<cliOptions> cliOptions;
	try {
		cliOptions = parseCLI(argc, argv);
	}
	catch (...) {
		std::cerr << "error parsing arguments";
		return 1;
	}

	world w(cliOptions->width, cliOptions->height);

	std::default_random_engine prng;
	if (cliOptions->seed < 0) {
		std::random_device r;
		prng.seed(r());
	} else {
		prng.seed(cliOptions->seed);
	}

	for (int i = 0; i < cliOptions->liveCount; i++) {
		w.set(prng(), prng(), true);
	}

	while (true) {
		render(w);
		w.step();
		std::this_thread::sleep_for(std::chrono::milliseconds(cliOptions->stepTime));
	}

	return 0;
}

std::unique_ptr<cliOptions> parseCLI(int argc, const char * argv[]) {
	TCLAP::CmdLine cmd("Conway's Game of Life", ' ', "0.1");
	TCLAP::ValueArg<int> widthArg("", "width", "width of world", false, 30, "int");
	cmd.add(widthArg);
	TCLAP::ValueArg<int> heightArg("", "height", "height of world", false, 30, "int");
	cmd.add(heightArg);
	TCLAP::ValueArg<int> liveCountArg("", "livecount", "live cells to start", false, 100, "int");
	cmd.add(liveCountArg);
	TCLAP::ValueArg<int> stepTimeArg("", "steptime", "time per step in milliseconds", false, 1000, "int");
	cmd.add(stepTimeArg);
	TCLAP::ValueArg<int> seedArg("", "seed", "seed for PRNG", false, -1, "int");
	cmd.add(seedArg);

	cmd.parse(argc, argv);

	auto options = std::make_unique<cliOptions>();
	options->width = widthArg.getValue();
	options->height = heightArg.getValue();
	options->liveCount = liveCountArg.getValue();
	options->stepTime = stepTimeArg.getValue();
	options->seed = seedArg.getValue();

	return options;
}

void render(world& w) {
	std::cout << string(80, '\n');

	for (int y = 0; y < w.getHeight(); y++) {
		for (int x = 0; x < w.getWidth(); x++) {
			if (w.get(x, y)) {
				std::cout << "*";
			}
			else {
				std::cout << " ";
			}
		}
		std::cout << "\n";
	}

	std::cout.flush();
}
