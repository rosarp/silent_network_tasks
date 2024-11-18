// Minimalistic WebSocket server for testing

Bun.serve({
  port: 4010,
  fetch(req, server) {
    // upgrade the request to a WebSocket
    if (server.upgrade(req)) {
      return; // do not return a Response
    }
    return new Response("Upgrade failed", { status: 500 });
  },
  websocket: {
    message(ws, message) {
      ws.send("Test Ws Server: " + message); // echo back the message
    }
  }, // handlers
});