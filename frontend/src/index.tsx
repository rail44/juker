/* @refresh reload */
import { render } from "solid-js/web";
import type { Component } from "solid-js";
import {
  createSignal,
  onMount,
  createResource,
  createEffect,
  useTransition,
  createMemo,
  Show,
  Suspense,
} from "solid-js";
import { v4 as uuidv4 } from "uuid";

import "./index.css";

declare global {
  var jukerYtLoadingPromise: Promise<void>;
  var YT: any;
}

const API_HOST = "localhost:8080";

const Player: Component<{
  videoId: string | null;
  socket: WebSocket;
  duration: number;
}> = (props) => {
  const [ready, setReady] = createSignal(false);
  const uuid = uuidv4();
  let player: any;
  onMount(() => {
    player = new YT.Player(uuid, {
      height: "360",
      width: "640",
      videoId: props.videoId,
      events: {
        onReady: () => {
          setReady(true);
        },
        onStateChange: (ev: any) => {
          if (ev.data === 1) {
            props.socket.send("ping");
          }

          if (ev.data === 0) {
            props.socket.send("feed");
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

interface SocketMessage {
  pointer: number;
  duration: number;
  queue: VideoRequest[];
}

const App: Component<{ socket: WebSocket }> = (props) => {
  const [initialized, setInitialized] = createSignal(false);
  const [videoId, setVideoId] = createSignal("");
  const [duration, setDuration] = createSignal(0);

  props.socket.addEventListener("message", (event) => {
    const message: SocketMessage = JSON.parse(event.data);
    const req = message.queue[message.pointer];
    if (!req) {
      return;
    }

    setVideoId(req.id);
    setDuration(message.duration);
    setInitialized(true);
  });

  return (
    <div>
      <Show when={initialized()}>
        <Player
          videoId={videoId()}
          duration={duration()}
          socket={props.socket}
        />
      </Show>
    </div>
  );
};

const socket = new WebSocket(`ws://${API_HOST}/socket`);

await window.jukerYtLoadingPromise;
render(
  () => (
    <Suspense>
      <App socket={socket} />
    </Suspense>
  ),
  document.getElementById("root") as HTMLElement
);
