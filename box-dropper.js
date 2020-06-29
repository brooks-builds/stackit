class BoxDropper {
  constructor() {
    this.location = createVector(0, 0);
    this.size = worldUnitSize + 5;
    this.color = color(255, 0, 0);
    this.velocity = createVector(5, 0);
  }

  render() {
    fill(this.color);
    rect(this.location.x, this.location.y, this.size, this.size);
  }

  update() {
    this.location.add(this.velocity);

    if (this.isCollidingWithEdge()) {
      this.velocity.mult(-1);
    }
  }

  isCollidingWithEdge() {
    return this.location.x + this.size >= width || this.location.x <= 0;
  }
}
