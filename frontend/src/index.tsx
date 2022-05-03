/* @refresh reload */
import { render } from "solid-js/web";
import type { Component } from "solid-js";
import { createSignal, onMount, createDeferred, For } from "solid-js";
import { v4 as uuidv4 } from "uuid";

import "./index.css";

const DUMMY_ID = "AlXGFHExSL4";

declare global {
  var YT: any;
}

const API_HOST = "http://localhost:8080/";

const Player: Component<{ videoId: string }> = ({ videoId }) => {
  const uuid = uuidv4();
  onMount(() => {
    const player = new YT.Player(uuid, {
      height: "360",
      width: "640",
      videoId,
      events: {
        onReady: () => {
          player.playVideo();
        },
        onStateChange: () => {},
      },
    });
  });

  return <div id={uuid}></div>;
};

const App: Component = () => {
  const [videoIds, setVideoId] = createSignal<string[]>([]);

  return (
    <div>
      <For each={videoIds()}>{(id) => <Player videoId={id} />}</For>
      {JSON.stringify(videoIds())}
      <button onClick={() => setVideoId((arr) => [...arr, DUMMY_ID])}>
        hoge
      </button>
    </div>
  );
};

export default App;

render(() => <App />, document.getElementById("root") as HTMLElement);
