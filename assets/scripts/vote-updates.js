function subscribe() {
  const scheme = location.protocol.startsWith("https") ? "wss" : "ws";
  const websocket = new WebSocket(
    `${scheme}://${window.location.host}/websocket`,
  );

  websocket.onmessage = (e) => {
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
