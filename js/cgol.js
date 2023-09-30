const minimist = require('minimist');
const World = require("./world.js");

const args = minimist(process.argv.slice(2));

const width = args.width || 30;
const height = args.height || 30;
const liveCount = args['live-count'] || 100;
const stepTime = args['step-time'] || 250;
// Not supporting seed like other implementations because JS doesn't natively suport it.

function render(world) {
  process.stdout.write("\x1Bc");

  for (let y = 0; y < world.height; y++) {
    for (let x = 0; x < world.width; x++) {
      process.stdout.write(world.get(x, y) ? "*" : " ");
    }
    process.stdout.write("\n");
  }
}

let w = new World(width, height);

for (let i = 0; i < liveCount; i++) {
  w.set(Math.floor(Math.random() * width), Math.floor(Math.random() * height), true);
}

function mainLoop() {
  render(w);
  w = w.step();
  setTimeout(mainLoop, stepTime);
}

mainLoop();
