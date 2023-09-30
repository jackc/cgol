const minimist = require('minimist');
const World = require("./world.js");

const args = minimist(process.argv.slice(2));

const width = args.width || 100;
const height = args.height || 100;
const liveCount = args['live-count'] || 100;
const stepCount = args['step-count'] || 1000;
// Not supporting seed like other implementations because JS doesn't natively suport it.

let w = new World(width, height);

for (let i = 0; i < liveCount; i++) {
  w.set(Math.floor(Math.random() * width), Math.floor(Math.random() * height), true);
}

const startTime = Date.now();
for (let i = 0; i < stepCount; i++) {
  w = w.step();
}
const endTime = Date.now();

console.log(`JS: ${(endTime - startTime) / 1000}`);
