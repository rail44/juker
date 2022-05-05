/* @refresh reload */
import { render } from "solid-js/web";
import type { Component } from "solid-js";
import {
  createSignal,
  onMount,
  createEffect,
  Show,
  Suspense,
  createMemo,
  untrack,
} from "solid-js";
import { v4 as uuidv4 } from "uuid";

import "./index.css";

declare global {
  var jukerYtLoadingPromise: Promise<void>;
  var YT: any;
}

// TODO: detect environment and switch endpoit for websocket
const WS_ENDPOINT = "wss://juker.onrender.com/socket";
// const WS_ENDPOINT = "ws://localhost:8080/socket";

function sendMessage(socket: WebSocket, message: SocketRequest) {
  socket.send(JSON.stringify(message));
}

const Player: Component<{
  playing: PlayingStatus;
  socket: WebSocket;
}> = (props) => {
  // TODO: filter frequently prop changing

  const position = createMemo(() => props.playing.position);
  const [ready, setReady] = createSignal(false);
  const uuid = uuidv4();
  let player: any;
  onMount(() => {
    player = new YT.Player(uuid, {
      height: "360",
      width: "640",
      playerVars: {
        controls: 0,
        disablekb: 1,
      },
      events: {
        onReady: () => {
          setReady(true);
        },
        onStateChange: (ev: any) => {
          console.log(ev.data);
          if (ev.data === 1) {
            sendMessage(props.socket, {
              type: "ping",
            });
          }

          if (ev.data === 0) {
            sendMessage(props.socket, {
              type: "feed",
              position: props.playing.position + 1,
            });
          }
        },
      },
    });
  });

  createEffect(() => {
    if (!ready()) {
      return;
    }

    position();
    player.loadVideoById(untrack(() => props.playing.id));
  });

  createEffect(() => {
    if (!ready()) {
      return;
    }

    if (Math.abs(player.getCurrentTime() - props.playing.duration) <= 1) {
      return;
    }

    player.seekTo(props.playing.duration);
  });

  return <div id={uuid}></div>;
};

interface VideoRequest {
  id: string;
}

type SocketRequest = Ping | Feed;

interface Ping {
  type: "ping";
}

interface Feed {
  type: "feed";
  position: number;
}

interface PlayingStatus {
  id: number;
  position: number;
  duration: number;
}

interface SocketResponse {
  playing: PlayingStatus | null;
  listeners: number;
  count: number;
}

const App: Component<{ socket: WebSocket }> = (props) => {
  const [playingStatus, setPlayingStatus] = createSignal<PlayingStatus | null>(
    null
  );
  const [listeners, setListeners] = createSignal(0);
  const [count, setCount] = createSignal(0);

  onMount(() => {
    props.socket.addEventListener("message", (event) => {
      console.log(event);

      const message: SocketResponse = JSON.parse(event.data);

      setPlayingStatus(message.playing);
      setListeners(message.listeners);
      setCount(message.count);
    });

    sendMessage(props.socket, { type: "ping" });
  });

  return (
    <div>
      <div>
        <Show when={playingStatus() !== null}>
          <Player playing={playingStatus()!} socket={props.socket} />
        </Show>
      </div>
      <div>listeners: {listeners()}</div>
      <Show when={playingStatus() === null}>
        <div>queue count: {count()}</div>
      </Show>
      <Show when={playingStatus() !== null}>
        <div>remains: {count() - playingStatus()!.position - 1}</div>
      </Show>
    </div>
  );
};

const socket = new WebSocket(WS_ENDPOINT);
// TODO: notify and reloading when error occuring on websocket
await new Promise((resolve) => socket.addEventListener("open", resolve));
await window.jukerYtLoadingPromise;

render(
  () => (
    <Suspense>
      <App socket={socket} />
    </Suspense>
  ),
  document.getElementById("root") as HTMLElement
);
