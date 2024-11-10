import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { createEffect, Suspense } from "solid-js";
import { createStore } from "solid-js/store";
import Cookies from "universal-cookie";
import "./app.css";
import Navbar from "./components/Navbar";

const APP_STATE_NAME = "AppState";
const cookieJar = new Cookies();

export default function App() {
  const cookies = cookieJar.get(APP_STATE_NAME);
  const [appState, setAppState] = createStore<{ isDarkMode: boolean }>(
    cookies || { isDarkMode: true },
  );

  createEffect(() => {
    if (appState.isDarkMode) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }

    const expiry = new Date();
    expiry.setFullYear(expiry.getFullYear() + 20);

    cookieJar.set(APP_STATE_NAME, appState, {
      sameSite: "strict",
      expires: expiry,
    });
  });

  const toggleDarkMode = () => {
    const new_state = {
      isDarkMode: !appState.isDarkMode,
    };

    setAppState(new_state);
  };

  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <Title>MCS</Title>
          <Navbar
            toggleDarkMode={toggleDarkMode}
            isDarkMode={appState.isDarkMode}
          />
          <Suspense>{props.children}</Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
