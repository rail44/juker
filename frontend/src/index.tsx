/* @refresh reload */
import { render } from "solid-js/web";
import type { Component } from "solid-js";
import { createSignal, onMount, createEffect, Show, Suspense } from "solid-js";
import { v4 as uuidv4 } from "uuid";

import "./index.css";

declare global {
  var jukerYtLoadingPromise: Promise<void>;
  var YT: any;
}

const API_HOST = "juker.onrender.com";

function sendMessage(socket: WebSocket, message: SocketRequest) {
  socket.send(JSON.stringify(message));
}

const Player: Component<{
  videoId: string | null;
  pointer: number;
  socket: WebSocket;
  duration: number;
}> = (props) => {
  // TODO: filter frequently prop changing

  const [ready, setReady] = createSignal(false);
  const uuid = uuidv4();
  let player: any;
  onMount(() => {
    player = new YT.Player(uuid, {
      height: "360",
      width: "640",
      videoId: props.videoId,
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
            sendMessage(props.socket, { type: "ping" });
          }

          if (ev.data === 0) {
            sendMessage(props.socket, {
              type: "feed",
              pointer: props.pointer + 1,
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

    player.loadVideoById(props.videoId);
  });

  createEffect(() => {
    if (!ready()) {
      return;
    }

    console.log(props.duration);
    player.seekTo(props.duration);
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
  pointer: number;
}

interface SocketReceived {
  pointer: number;
  duration: number;
  queue: VideoRequest[];
}

const App: Component<{ socket: WebSocket }> = (props) => {
  const [initialized, setInitialized] = createSignal(false);
  const [videoId, setVideoId] = createSignal("");
  const [pointer, setPointer] = createSignal(0);
  const [duration, setDuration] = createSignal(0);

  onMount(() => {
    props.socket.addEventListener("message", (event) => {
      console.log(event);

      const message: SocketReceived = JSON.parse(event.data);
      if (message.pointer === null) {
        return;
      }

      const req = message.queue[message.pointer];
      setVideoId(req.id);
      setDuration(message.duration);
      setPointer(message.pointer);
      setInitialized(true);
    });

    sendMessage(props.socket, { type: "ping" });
  });

  return (
    <div>
      <Show when={initialized()}>
        <Player
          videoId={videoId()}
          duration={duration()}
          socket={props.socket}
          pointer={pointer()}
        />
      </Show>
    </div>
  );
};

const socket = new WebSocket(`wss://${API_HOST}/socket`);
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
