// var addon = require('../native');

// console.log(addon.hello());

module.exports = {
  fill(buffer, x, y, width, height, [r, g, b]) {
    for (let i = 0; i < width; i++) {
      for (let j = 0; j < height; j++) {
        const index = ((y + j) * width + x + i) * 4;
        buffer[index] = r;
        buffer[index + 1] = g;
        buffer[index + 2] = b;
        buffer[index + 3] = 255;
      }
    }
  }
}