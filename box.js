class Box {
  constructor(location, velocity, username, usersColor) {
    this.location = createVector(location.x + 2.5, location.y);
    this.velocity = velocity;
    this.velocity.y = 2;
    this.color = color(usersColor);
    this.size = worldUnitSize;
    // see lines 46 - line 10 + 11 bandaid the differences in classes at the moment
    // our classes probably need a base object to inherit from so we can unify this stuff and reduce code duplication
    this.height = this.size;
    this.width = this.size;
    this.username = username;
    this.isLanded = false;
    this.isDead = false;
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

  update(velocity = this.velocity) {
    if (this.isDead) {
      // dead stuff falls down faster and faster
      this.velocity.y *= 1.05;
      this.location.add(this.velocity);
      return;
    }

    if (this.isCollidingWithEdge() && !this.isLanded) {
      if (this.location.x + this.size >= width) {
        this.location.x = width - this.size;
      } else if (this.location.x <= 0) {
        this.location.x = 0;
      }
      this.velocity.x *= -1;
    } else if (
      (this.isCollidingWithEdge() || this.isColliding(boxDropper)) &&
      this.isLanded
    ) {
      this.isLanded = false;
      this.isDead = true;
      this.velocity.mult(0);
      // setTimeout(() => {
      this.velocity.y = 1.0; // timeout optional, I personally think them dropping in line with the edge on hit looks better - bare
      // }, 3000);
    }

    this.location.add(velocity);
  }

  isCollidingWithEdge() {
    return this.location.x + this.size >= width || this.location.x <= 0;
  }

  // this can check any object with an x, y, width, and height IE: box and platform
  isColliding(target) {
    if (
      this.location.x < target.location.x + target.width &&
      this.location.x + this.size > target.location.x &&
      this.location.y < target.location.y + target.height &&
      this.location.y + this.size > target.location.y
    ) {
      return true;
    }

    return false;
  }

  isOffScreen() {
    return this.location.y > height;
  }
}
