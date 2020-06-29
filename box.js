class Box {
  constructor(location, velocity) {
    this.location = location;
    this.velocity = velocity;
    this.velocity.y = 2;
    this.color = color(0, 255, 0);
    this.size = worldUnitSize;
  }

  render() {
    fill(this.color);
    rect(this.location.x, this.location.y, this.size, this.size);
  }

  update() {
    this.location.add(this.velocity);

    if (this.isCollidingWithEdge()) {
      this.velocity.x *= -1;
    }
  }

  isCollidingWithEdge() {
    return this.location.x + this.size >= width || this.location.x <= 0;
  }
}
