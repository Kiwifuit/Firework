/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{tsx,ts}"],
  theme: {
    extend: {
      fontFamily: {
        mono: "FiraCode Nerd Font",
        sans: "Inter"
      },
    },
    colors: {
      // Dark Mode
      "dark-bg": "#202020",
      "dark-fg": "#eaeaea",
      "dark-accent": "#303030",

      // Light Mode
      "light-bg": "#ebebeb",
      "light-fg": "#1c1c1c",
      "light-accent": "#cccccc",

      // ServerListItem component
      "light-server-description": "#3b3b3b",
      "light-server-background": "#dedcdc",
      "dark-server-description": "#a3a2a2",
      "dark-server-background": "#404040",

      // Server Dashboard component
      "dark-dashboard-hover":"#404040",
      "light-dashboard-hover":"#dddddd",

      // Server Online/Offline indicator
      "server-offline": "#d62f2f",
      "server-online": "#2ed154",

      // Dashboard: Logs & Console
      "dark-dashboard-title": "#252525",
      "dark-dashboard-body": "#303030",
      "dark-dashboard-button": "#404040",
      "dark-dashboard-text": "#505050",
      "dark-placeholder-text": "#cacaca",

      "light-dashboard-title": "#a0a0a0",
      "light-dashboard-body": "#909090",
      "light-dashboard-button": "#808080",
      "light-dashboard-text": "#707070",
      "light-placeholder-text": "#3c3c3c"
    },
  },
  plugins: [],
  darkMode: "class",
};
