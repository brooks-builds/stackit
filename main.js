const webSocket = new WebSocket("ws://localhost:8080");

webSocket.onmessage = (event) => console.log(event);
