class Water {
  constructor() {
    this.xspacing = 2; // Distance between each horizontal location
    this.w = width + this.xspacing; // Width of entire wave
    this.theta = 0.0; // Start angle at 0
    this.amplitude = 5; // Height of wave
    this.period = 400.0; // How many pixels before the wave repeats
    this.dx = (TWO_PI / this.period) * this.xspacing; // Value for incrementing x
    this.yvalues = new Array(floor(this.w / this.xspacing)); // Using an array to store height values for the wave
    this.centerLocationY = height + 2 - this.amplitude / 2;
    this.color = color(0, 0, 255, 10);
  }

  update() {
    this.calcWave();
  }

  render() {
    noStroke();
    // fill(this.color);
    // A simple way to draw the wave with an ellipse at each location
    for (let x = 0; x < this.yvalues.length; x++) {
      let alpha = (((this.yvalues[x] - -this.amplitude) * (150 - 50)) / (this.amplitude - -this.amplitude)) + 50;
      let hmod = (((this.yvalues[x] - -this.amplitude) * (0 - 2)) / (this.amplitude - -this.amplitude)) + 2;
      fill(0, 255, 255, alpha);
      ellipse(x * this.xspacing, this.centerLocationY + this.yvalues[x] + -hmod, 8, 32);
      let color = (((this.yvalues[x] - -this.amplitude) * (80 - 255)) / (this.amplitude - -this.amplitude)) + 255;
      fill(0, 0, color, alpha);
      ellipse(x * this.xspacing, this.centerLocationY + this.yvalues[x], 16, 32);
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
