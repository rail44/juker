/* @refresh reload */
import { render } from "solid-js/web";
import type { Component } from "solid-js";
import { createSignal, onMount, For } from "solid-js";
import { v4 as uuidv4 } from "uuid";

import "./index.css";

const DUMMY_ID = "AlXGFHExSL4";

declare global {
  var YT: any;
}

const Player: Component<{ videoId: string }> = ({ videoId }) => {
  onMount(() => {
    const player = new YT.Player("player", {
      height: "360",
      width: "640",
      videoId,
      events: {
        onReady: () => {},
        onStateChange: () => {},
      },
    });
  });

  return <div id={}></iframe>;
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
