class BoxDropper {
  constructor() {
    this.size = worldUnitSize + 5;
    this.height = this.size;
    this.width = this.size;
    // start closer to the middle
    this.location = createVector(width / 2 - this.size / 2, 0);
    this.color = color(255, 0, 0);
    // randomize the initial direction
    this.velocity = createVector(random([-5, 5]), 0);
  }

  render() {
    render_outlined_rect(
      this.location.x,
      this.location.y,
      this.size,
      this.size,
      this.color
    );
  }

  update() {
    this.location.add(this.velocity);

    // TODO: this check and func call is dupe for dropper, platform, and box. make it a single function somewhere.
    if (this.isCollidingWithEdge()) {
      if (this.location.x + this.size >= width) {
        this.location.x = width - this.size;
      } else if (this.location.x <= 0) {
        this.location.x = 0;
      }

      this.velocity.mult(-1);
    }
  }

  isCollidingWithEdge() {
    return this.location.x + this.size >= width || this.location.x <= 0;
  }
}
