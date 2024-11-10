import { Title } from "@solidjs/meta";
import { FaSolidPlus } from "solid-icons/fa";
import { createResource, For } from "solid-js";
import ServerListItem from "~/components/ServerListItem";

const API_ENDPOINT = "http://localhost:3030";
type Server = {
  id: string;
  online: boolean;
  display_name: string;
  description: string;
  players: {
    active: number;
    total: number;
  };
  software: string;
  modpack: string | null;
};

export default function Home() {
  let [servers] = createResource(fetchServers);

  return (
    <main class="mt-32 grid w-screen place-items-center">
      <Title>Servers</Title>
      <div class="w-1/2 font-extralight">
        <div class="mb-5 flex flex-row">
          <h1 class="grow text-4xl font-bold">Servers:</h1>
          <a
            class="grid rounded bg-light-dashboard-button px-3 dark:bg-dark-dashboard-button"
            href="/new"
          >
            <FaSolidPlus class="m-auto" />
          </a>
        </div>
        <For each={servers()} fallback={<NoServerFallback />}>
          {(data) => (
            <ServerListItem
              id={data.id}
              online={data.online}
              display_name={data.display_name}
              description={data.description}
              players={data.players}
              modpack={data.modpack}
              software={data.software}
            />
          )}
        </For>
      </div>
    </main>
  );
}

async function fetchServers(): Promise<Server[]> {
  let resp = await fetch(`${API_ENDPOINT}/servers`);

  if (!resp.ok) {
    console.error(`Failed to fetch servers: ${resp.status} ${resp.statusText}`);
  }

  return await resp.json();
}

function NoServerFallback() {
  return (
    <div class="grid items-center justify-center py-32">
      <p class="italic">
        No servers listed. Create a server by pressing the{" "}
        <span class="font-mono not-italic">+</span> button
      </p>
    </div>
  );
}
