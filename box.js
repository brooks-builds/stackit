class Box {
  constructor(location, velocity, username, usersColor) {
    this.location = location;
    this.velocity = velocity;
    this.velocity.y = 2;
    this.color = color(usersColor);
    this.size = worldUnitSize;
    this.username = username;
    this.isLanded = false;
    this.isDead = false;
  }

  render() {
    render_outlined_rect(this.location.x, this.location.y, this.size, this.size, this.color);
  }

  update(velocity = this.velocity) {
    this.location.add(velocity);

    if (this.isDead) {
      this.location.add(this.velocity);
      this.velocity.y *= 1.05;
      return;
    }

    if (this.isCollidingWithEdge() && !this.isLanded) {
      this.velocity.x *= -1;
    } else if (this.isCollidingWithEdge() && this.isLanded) {
      this.isLanded = false;
      this.velocity.mult(0);
      this.isDead = true;
      setTimeout(() => {
        this.velocity.y = 1.0;
      }, 3000);
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

  collideWithLandedBox(landedBoxes) {
    for (let landedBox of landedBoxes) {
      if (
        this.location.x < landedBox.location.x + landedBox.size &&
        this.location.x + this.size > landedBox.location.x &&
        this.location.y < landedBox.location.y + landedBox.size &&
        this.location.y + this.size > landedBox.location.y
      ) {
        this.isFalling = false;
        return true;
      }
    }
    return false;
  }

  isOffScreen() {
    return this.location.y > height;
  }
}
