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
  parser.on("--step-count INTEGER", Integer, "Number of steps to run") do |value|
    options[:step_count] = value
  end
  parser.on("--seed INTEGER", Integer, "Seed for RNG") do |value|
    options[:seed] = value
  end
end.parse!

width = options.fetch(:width, 100)
height = options.fetch(:height, 100)
live_count = options.fetch(:live_count, 100)
step_count = options.fetch(:step_count, 1_000)
seed = options.fetch(:seed, 0)
srand(seed)

w = World.new(width, height)

live_count.times do
  w.set(rand(w.width), rand(w.height), true)
end

start_time = Time.now
step_count.times do |n|
  w = w.step
end
end_time = Time.now

duration = end_time - start_time
puts "Ruby: #{duration}"
