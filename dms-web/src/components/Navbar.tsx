import { A } from "@solidjs/router";
import { BiSolidCog, BiSolidHomeAlt2, BiSolidInfoCircle } from "solid-icons/bi";
import { FiMoon, FiSun } from "solid-icons/fi";

export default function Navbar(props: {
  toggleDarkMode: () => void;
  isDarkMode: boolean;
}) {
  return (
    <nav>
      <ul class="flex gap-4 bg-light-accent px-5 py-3 text-xl dark:bg-dark-accent">
        <li class="grow">
          <A href="/" class="flex w-fit items-center">
            <BiSolidHomeAlt2 />
            Home
          </A>
        </li>
        <li>
          <A href="/about" class="flex items-center">
            <BiSolidInfoCircle />
            About
          </A>
        </li>
        <li>
          <A href="/options" class="flex items-center">
            <BiSolidCog />
            Options
          </A>
        </li>
        <li>
          <button onclick={props.toggleDarkMode}>
            {props.isDarkMode ? <FiSun /> : <FiMoon />}
          </button>
        </li>
      </ul>
    </nav>
  );
}
