const webSocket = new WebSocket("ws://localhost:8080");
let worldUnitSize = 30;
let boxDropper;
const boxes = [];
let platform;
let score;

function setup() {
  createCanvas(1920, 1080);

  boxDropper = new BoxDropper();
  platform = new Platform();
  webSocket.onmessage = (event) => {
    const data = JSON.parse(event.data);
    boxes.push(
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
  clear();
  // update all the things
  boxDropper.update();
  boxes.forEach((box) => {
    box.update();
    if (box.points && box.collideWithPlatform(platform)) {
      score.addScore(box.username);
      box.points -= 1;
    }
  });

  // draw all the things
  drawBackground();
  boxes.forEach((box) => box.render());
  boxDropper.render();
  platform.render();
}

function drawBackground() {
  fill(0, 0, 255);
  rect(0, height - worldUnitSize, width, worldUnitSize);
}
