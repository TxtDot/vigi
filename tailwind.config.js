const plugin = require("tailwindcss/plugin");

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      animation: {
        "load-pulse": "pulse 0.7s linear infinite",
      },
    },
  },
  plugins: [
    plugin(({ addUtilities }) => {
      let utilities = {};
      for (let i = 0; i <= 100; i += 5) {
        utilities[`.color-vigi-${i}`] = {
          color: `color-mix(in var(--colorspace), var(--max-text) ${i}%, var(--min-text))`,
          "background-color": `color-mix(in var(--colorspace), var(--max-bg) ${i}%, var(--min-bg))`,
        };
      }

      addUtilities(utilities);
    }),
  ],
};
