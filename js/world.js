class World {
  constructor(width, height) {
    this.width = width;
    this.height = height;
    this.cells = new Array(width * height).fill(false);
  }

  step() {
    const w2 = new World(this.width, this.height);

    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        const neighborCount = this.countNeighbors(x, y);
        let newValue = null;

        if (this.get(x, y)) {
          newValue = neighborCount === 2 || neighborCount === 3;
        } else {
          newValue = neighborCount === 3;
        }

        w2.set(x, y, newValue);
      }
    }

    return w2;
  }

  set(x, y, val) {
    const idx = this.idxFromCoord(x, y);
    this.cells[idx] = val;
  }

  get(x, y) {
    const idx = this.idxFromCoord(x, y);
    return this.cells[idx];
  }

  countNeighbors(x, y) {
    const neighborOffsets = [
      [-1, -1], [0, -1], [1, -1],
      [-1, 0], [1, 0],
      [-1, 1], [0, 1], [1, 1]
    ];

    let count = 0;

    for (const [dx, dy] of neighborOffsets) {
      if (this.get(x + dx, y + dy)) {
        count++;
      }
    }

    return count;
  }

  idxFromCoord(x, y) {
    x = (x + this.width) % this.width;
    y = (y + this.height) % this.height;

    return y * this.width + x;
  }
}

module.exports = World;
