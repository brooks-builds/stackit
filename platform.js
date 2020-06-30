class Platform {
  constructor() {
    this.height = worldUnitSize;
    this.width = worldUnitSize * 5;
    this.location = createVector(
      width / 2 - this.width / 2,
      height - this.height
    );
    // this.width = width;
    // this.location = createVector(0, height - this.height);
    this.color = color(255, 255, 0);
  }

  render() {
    fill(this.color);
    rect(this.location.x, this.location.y, this.width, this.height);
  }
}
