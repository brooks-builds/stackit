class Water {
  constructor() {
    this.xspacing = 8; // Distance between each horizontal location
    this.w = width + this.xspacing; // Width of entire wave
    this.theta = 0.0; // Start angle at 0
    this.amplitude = 5; // Height of wave
    this.period = 250.0; // How many pixels before the wave repeats
    this.dx = (TWO_PI / this.period) * this.xspacing; // Value for incrementing x
    this.yvalues = new Array(floor(this.w / this.xspacing)); // Using an array to store height values for the wave
    this.centerLocationY = height - this.amplitude / 2;
    this.color = color(0, 0, 255, 10);
  }

  update() {
    this.calcWave();
  }

  render() {
    noStroke();
    fill(this.color);
    // A simple way to draw the wave with an ellipse at each location
    for (let x = 0; x < this.yvalues.length; x++) {
      ellipse(
        x * this.xspacing,
        this.centerLocationY + this.yvalues[x],
        24,
        24
      );
    }
  }

  calcWave() {
    // Increment theta (try different values for
    // 'angular velocity' here)
    this.theta += 0.02;

    // For every x value, calculate a y value with sine function
    let x = this.theta;
    for (let i = 0; i < this.yvalues.length; i++) {
      this.yvalues[i] = sin(x) * this.amplitude;
      x += this.dx;
    }
  }
}
