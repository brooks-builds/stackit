class Platform {
  constructor() {
    this.height = worldUnitSize;
    this.width = worldUnitSize * 5;
    this.location = createVector(
      width / 2 - this.width / 2,
      height - this.height
    );
    this.velocity = createVector(random([-1, 1]), 0);
    this.color = color(255, 255, 0);
  }

  resize() {
    this.location.y = height - this.height;
  }

  render() {
    render_outlined_rect(
      this.location.x,
      this.location.y,
      this.width,
      this.height,
      this.color
    );
  }

  update() {
    this.location.add(this.velocity);
    this.collideWithWalls();
  }

  collideWithWalls() {
    if (this.location.x + this.width >= width) {
      this.location.x = width - this.width;
      this.velocity.mult(-1);
    } else if (this.location.x <= 0) {
      this.location.x = 0;
      this.velocity.mult(-1);
    }

    // if (this.location.x < -this.width) {
    //   this.location.x = -this.width;
    //   this.velocity.mult(-1);
    // } else if (this.location.x > width + this.width) {
    //   this.location.x = this.width + this.width;
    //   this.velocity.mult(-1);
    // }
  }
}
