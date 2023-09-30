require 'optparse'
require_relative "world"

options = {}
OptionParser.new do |parser|
  parser.banner = "Usage: cgol.rb [options]"

  parser.on("--width INTEGER", Integer, "Width") do |value|
    options[:width] = value
  end
  parser.on("--height INTEGER", Integer, "Height") do |value|
    options[:height] = value
  end
  parser.on("--live-count INTEGER", Integer, "Live cell count to start") do |value|
    options[:live_count] = value
  end
  parser.on("--step-time INTEGER", Integer, "Step time in milliseconds") do |value|
    options[:step_time] = value
  end
  parser.on("--seed INTEGER", Integer, "Seed for RNG") do |value|
    options[:seed] = value
  end
end.parse!

width = options.fetch(:width, 30)
height = options.fetch(:height, 30)
live_count = options.fetch(:live_count, 100)
step_time = options.fetch(:step_time, 250).to_f / 1000
seed = options.fetch(:seed, 0)
srand(seed)

def render(world)
  puts "\033c"

  world.height.times do |y|
    world.width.times do |x|
      print world.get(x, y) ? "*" : " "
    end
    puts
  end
end

w = World.new(width, height)

live_count.times do
  w.set(rand(w.width), rand(w.height), true)
end

loop do
  render(w)
  w = w.step
  sleep(step_time)
end
