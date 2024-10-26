import { Title } from "@solidjs/meta";
import { FaSolidPlus } from "solid-icons/fa";
import {
  createEffect,
  createResource,
  createSignal,
  For,
  Match,
  Show,
  Switch,
} from "solid-js";

const API_ENDPOINT = "http://localhost:3030";
type ServerEntry = {
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
  // const [isOpen, setOpen] = createSignal(false);
  const [serverList, setServerLists] = createSignal(new Array(5));
  const [servers] = createResource(serverList, fetch_servers);

  createEffect(() => {
    console.log(serverList());
  }, servers());

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
        <Show when={servers.loading}>
          <div>
            <p>Loading...</p>
          </div>
        </Show>
        <Switch>
          <Match when={servers.error}>
            <div>
              <p>An error occurred while loading servers: "{servers.error}"</p>
            </div>
          </Match>
          <Match when={serverList()}>
            <For each={serverList()}>
              {(data) => <p>{JSON.stringify(data)}</p>}
            </For>{" "}
          </Match>
        </Switch>

        {/* <ServerListItem
          id="server-1"
          online={true}
          display_name="Server 01 Name"
          description="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed viverra lorem a bibendum vestibulum. Donec tincidunt at nulla aliquam consectetur. Nullam fringilla libero sit amet quam eleifend, id tempor nunc venenatis. Ut euismod sapien quis quam ullamcorper, et consectetur elit imperdiet. Etiam a odio ut ipsum molestie ornare. Etiam rutrum leo non nibh scelerisque auctor. Vivamus eu erat eget justo luctus condimentum. Vivamus hendrerit mi eget lacus imperdiet placerat. Maecenas molestie elit sit amet sem consectetur, vitae tincidunt sapien ullamcorper. Sed fringilla, diam sit amet faucibus tristique, elit ipsum pulvinar velit, eget tincidunt justo ante eget lacus. Proin dapibus venenatis blandit. Donec ut tortor luctus, dignissim felis nec, cursus risus. Cras a ultricies enim. Etiam et elit ut tellus dapibus suscipit. Nulla eget condimentum nisi, eu mattis elit. "
          players={{ total: 20, active: 15 }}
          modpack="test"
          software="Forge 1.2.3 for Minecraft 1.20.1"
        />
        <ServerListItem
          id="server-2"
          online={true}
          display_name="Server 02 Name"
          description=""
          players={{ total: 20, active: 15 }}
          modpack="test2"
          software="Forge 1.2.3 for Minecraft 1.20.1"
        />
        <ServerListItem
          id="server-3"
          online={true}
          display_name="Server 03 Name"
          description="Server 03 Description"
          players={{ total: 20, active: 10 }}
          modpack="test3"
          software="Neoforge 1.2.3 for Minecraft 1.20.1"
        />
        <ServerListItem
          id="server-4"
          online={false}
          display_name="Server 04 Name"
          description="Server 04 Description"
          players={{ total: 20, active: 0 }}
          modpack={null}
          software="Quilt 1.2.3 for Minecraft 1.20.1"
        />
        <ServerListItem
          id="server-5"
          online={true}
          display_name="Server 05 Name"
          description="Server 05 Description"
          players={{ total: 20, active: 20 }}
          modpack={null}
          software="Fabric 1.2.3 for Minecraft 1.20.1"
        /> */}
      </div>
    </main>
  );
}

async function fetch_servers() {
  const response = await fetch(`${API_ENDPOINT}/servers`);

  return response.json();
}
