/** @type {import('tailwindcss').Config} */
import defaultTheme from "tailwindcss/defaultTheme";

export default {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      fontFamily: {
        display: ["Dela Gothic One", ...defaultTheme.fontFamily.sans],
        sans: ["Chivo Variable", ...defaultTheme.fontFamily.sans],
      },
      borderWidth: {
        1: ["0.5px"],
      },
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: [
      {
        dark: {
          ...require("daisyui/src/theming/themes")["dark"],
          primary: "cba6f7", //mauve
          secondary: "f38ba8", //rose
          neutral: "313244",
          "base-content": "f5e0dc", //rosewater
          "base-100": "1e1e2e",
          "base-200": "181825",
          "base-300": "11111b",
        },
      },
    ],
  },
};
