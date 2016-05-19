#include "stdafx.h"

#include <iostream>
#include<memory>
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
	std::cout << "Hello, World!\n";
	std::cout << w.getWidth() << ", " << w.getHeight() << std::endl;
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
	TCLAP::ValueArg<int> stepTimeArg("", "steptime", "time per step in milliseconds", false, 250, "int");
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
