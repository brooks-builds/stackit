class Box {
  constructor(location, velocity, username, usersColor) {
    this.location = location;
    this.velocity = velocity;
    this.velocity.y = 2;
    this.color = color(usersColor);
    this.size = worldUnitSize;
    this.isFalling = true;
    this.username = username;
    this.points = 1;
  }

  render() {
    fill(this.color);
    rect(this.location.x, this.location.y, this.size, this.size);
  }

  update() {
    if (this.isFalling) {
      this.location.add(this.velocity);

      if (this.isCollidingWithEdge()) {
        this.velocity.x *= -1;
      }
    }
  }

  isCollidingWithEdge() {
    return this.location.x + this.size >= width || this.location.x <= 0;
  }

  collideWithPlatform(platform) {
    if (
      this.location.x < platform.location.x + platform.width &&
      this.location.x + this.size > platform.location.x &&
      this.location.y < platform.location.y + platform.height &&
      this.location.y + this.size > platform.location.y
    ) {
      this.isFalling = false;
      return true;
    }
    return false;
  }
}
