async function smokeTest() {
  const response = await fetch("/api/smoke");

  if (response.status == 200) {
    location.reload();
  }
}

async function subscribe() {
  const scheme = location.protocol.startsWith("https") ? "wss" : "ws";
  const websocket = new WebSocket(
    `${scheme}://${window.location.host}/websocket`,
  );

  websocket.onclose = () => {
    setInterval(async () => {
      await smokeTest();
    }, 500);
  };
}

subscribe();
