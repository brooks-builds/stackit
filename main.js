const webSocket = new WebSocket("ws://localhost:8080");
let worldUnitSize = 30;
let boxDropper;
let fallingBoxes = [];
const landedBoxes = [];
let platform;
let score;
let water;
let test_username = "$Test$";

function setup() {
  // create a canvas with the visible extents of our browser same as inner width and height.
  // if you are using this in OBS or something, then set the browser source size to 1920x1080 or whatever and this will use that
  // window resizing changes the canvas size, and while this works correctly, we aren't actually scaling things to make it useful
  createCanvas(windowWidth, windowHeight);

  boxDropper = new BoxDropper();
  platform = new Platform();
  webSocket.onmessage = (event) => {
    const data = JSON.parse(event.data);
    let boxColor = data.color;

    if (boxColor == "no color" && data.username != test_username) {
      webSocket.send(
        "customize your box color by changing your user color with /color <color in hex>"
      );
      boxColor = "white";
    }

    fallingBoxes.push(
      new Box(
        boxDropper.location.copy(),
        boxDropper.velocity.copy(),
        data.username,
        data.color
      )
    );
  };
  score = new Score();
  water = new Water();
}

function draw() {
  clear();
  background("#222222");
  // update all the things
  boxDropper.update();
  platform.update();
  fallingBoxes.forEach((box) => {
    box.update();
    if (
      box.isColliding(platform) ||
      landedBoxes.some((landedBox) => box.isColliding(landedBox))
    ) {
      if (box.username != test_username) {
        score.addScore(box.username);
        webSocket.send(
          `${box.username} scored! They now have ${
            score.scores[box.username]
          } points`
        );
      }
      landedBoxes.push(box);
      box.isLanded = true;
    }
  });
  fallingBoxes = fallingBoxes.filter((box) => {
    if (box.isOffScreen()) return false;

    return !box.isLanded;
  });
  water.update();

  // draw all the things
  landedBoxes.forEach((box) => {
    box.update(platform.velocity);
    box.render();
  });
  fallingBoxes.forEach((box) => box.render());
  boxDropper.render();
  platform.render();
  water.render();
}

function render_outlined_rect(x, y, w, h, color) {
  strokeWeight(1);
  stroke("black");
  fill(color);
  rect(x, y, w, h);
}

function mouseClicked() {
  fallingBoxes.push(
    new Box(
      boxDropper.location.copy(),
      boxDropper.velocity.copy(),
      test_username,
      random_rgb_color()
    )
  );
}

function random_rgb_color() {
  return color(random(256), random(256), random(256));
}

function windowResized() {
  resizeCanvas(windowWidth, windowHeight);
  water.resize();
  platform.resize();
}
