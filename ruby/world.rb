class World
  attr_reader :width, :height

  def initialize(width, height)
    @width = width
    @height = height
    @cells = Array.new(width * height, false)
  end

  def step
    w2 = World.new(width, height)

    height.times do |y|
      width.times do |x|
        new_value = nil
        neighbor_count = count_neighbors(x, y)
        if get(x, y)
          new_value = neighbor_count == 2 || neighbor_count == 3
        else
          new_value = neighbor_count == 3
        end
        w2.set(x, y, new_value)
      end
    end

    w2
  end

  def set(x, y, val)
    idx = idx_from_coord(x, y)
    @cells[idx] = val
  end

  def get(x, y)
    idx = idx_from_coord(x, y)
    @cells[idx]
  end

  def count_neighbors(x, y)
    [
      get(x-1, y-1),
      get(x, y-1),
      get(x+1, y-1),

      get(x-1, y),
      get(x+1, y),

      get(x-1, y+1),
      get(x, y+1),
      get(x+1, y+1),
    ].count { _1 }
  end

  # idxFromCoord takes x and y coordinates and returns the index in w.cells.
  # Coordinates wrap the boundaries of the world. e.g. Given World with a
  # width of 10, then an x coordinate of -1 should be equal to 9.
  def idx_from_coord(x, y)
    x = x % width
    x += width if x < 0
    y = y % height
    y += height if y < 0

    y * width + x
  end
end
