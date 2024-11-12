import { Match, Switch } from "solid-js";

export default function ServerListItem(props: {
  id: string;
  status: string;
  display_name: string;
  description: string;
  players: {
    active: number;
    total: number;
  };
  software: string;
  modpack: string | null;
}) {
  return (
    <div class="mb-3 rounded-md bg-light-server-background p-2 dark:bg-dark-server-background">
      <a href={`/server/${props.id}/`} class="grid grid-cols-2">
        <p class="ml-3 text-xl font-light">{props.display_name}</p>
        <p class="col-start-1 row-span-2 ml-6 truncate text-light-server-description dark:text-dark-server-description">
          {props.description ? props.description : <i>No description</i>}
        </p>
        <Switch>
          <Match when={props.status == "online"}>
            <ServerOnline />
          </Match>
          <Match when={props.status == "offline"}>
            <ServerOffline />
          </Match>
          <Match when={props.status == "building"}>
            <ServerBuilding />
          </Match>
        </Switch>
        <p class="mr-3 text-right align-middle">
          Players: {props.players.active || "0"} / {props.players.total}
        </p>
        <p class="col-start-2 mr-3 text-right align-middle">
          Server: {props.software}
        </p>
      </a>
    </div>
  );
}

function ServerOnline() {
  return (
    <p class="col-start-2 row-start-1 mr-3 text-right">
      Status:
      <span class="mx-1 text-server-online">●</span>
      <span>Online</span>
    </p>
  );
}

function ServerOffline() {
  return (
    <p class="col-start-2 row-start-1 mr-3 text-right">
      Status:
      <span class="mx-1 text-server-offline">●</span>
      <span>Offline</span>
    </p>
  );
}

function ServerBuilding() {
  return (
    <p class="col-start-2 row-start-1 mr-3 text-right">
      Status:
      <span class="text-server-building mx-1">●</span>
      <span>Building</span>
    </p>
  );
}
