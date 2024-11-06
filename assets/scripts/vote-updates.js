function subscribe() {
  console.log("Connecting");

  const startTime = Date.now();

  const scheme = location.protocol.startsWith("https") ? "wss" : "ws";
  const websocket = new WebSocket(
    `${scheme}://${window.location.host}/websocket`,
  );

  websocket.onmessage = (e) => {
    const timeSinceStart = Date.now() - startTime;

    // skip re proccessing start message
    if (timeSinceStart < 500) {
      return;
    }

    htmx.swap("#vote-results", e.data, { swapStyle: "outerHTML" });
  };

  websocket.onopen = () => {
    console.log("Connection opened");
  };

  websocket.onclose = () => {
    console.log("Disconnected");
    setTimeout(() => subscribe(), 10000);
  };
}

subscribe();
