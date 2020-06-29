const webSocket = new WebSocket("ws://localhost:8080");
let worldUnitSize = 30;
let boxDropper;
const boxes = [];

function setup() {
  createCanvas(1920, 1080);

  boxDropper = new BoxDropper();
  webSocket.onmessage = (event) =>
    boxes.push(new Box(boxDropper.location.copy(), boxDropper.velocity.copy()));
}

function draw() {
  clear();
  // update all the things
  boxDropper.update();
  boxes.forEach((box) => box.update());

  // draw all the things
  drawBackground();
  boxes.forEach((box) => box.render());
  boxDropper.draw();
}

function drawBackground() {
  fill(0, 0, 255);
  rect(0, height - worldUnitSize, width, worldUnitSize);
}
