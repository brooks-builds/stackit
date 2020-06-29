const webSocket = new WebSocket("ws://localhost:8080");
webSocket.onmessage = (event) => console.log(event);

let worldUnitSize = 30;

function setup() {
  createCanvas(1920, 1080);
}

function draw() {
  drawBackground();
}

function drawBackground() {
  fill(0, 0, 255);
  rect(0, height - worldUnitSize, width, worldUnitSize);
}
