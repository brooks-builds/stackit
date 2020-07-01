const webSocket = new WebSocket("ws://localhost:8080");
let worldUnitSize = 30;
let boxDropper;
let fallingBoxes = [];
const landedBoxes = [];
let platform;
let score;

function setup() {
  createCanvas(1920, 1080);

  boxDropper = new BoxDropper();
  platform = new Platform();
  webSocket.onmessage = (event) => {
    const data = JSON.parse(event.data);
    let boxColor = data.color;

    if (boxColor == "no color") {
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
}

function draw() {
  console.log(fallingBoxes.length);
  clear();
  // update all the things
  boxDropper.update();
  fallingBoxes.forEach((box) => {
    box.update();
    if (
      box.collideWithPlatform(platform) ||
      box.collideWithLandedBox(landedBoxes)
    ) {
      score.addScore(box.username);
      webSocket.send(
        `${box.username} scored! They now have ${
          score.scores[box.username]
        } points`
      );
      landedBoxes.push(box);
      box.isLanded = true;
    }
  });
  fallingBoxes = fallingBoxes.filter((box) => {
    if (box.isOffScreen()) return false;

    return !box.isLanded;
  });

  // draw all the things
  drawBackground();
  landedBoxes.forEach((box) => box.render());
  fallingBoxes.forEach((box) => box.render());
  boxDropper.render();
  platform.render();
}

function drawBackground() {
  fill(0, 0, 255);
  rect(0, height - worldUnitSize, width, worldUnitSize);
}
