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
};
