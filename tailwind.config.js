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
        let text = `color-mix(in var(--colorspace), var(--max-text) ${i}%, var(--min-text))`;
        let bg = `color-mix(in var(--colorspace), var(--max-bg) ${i}%, var(--min-bg))`;

        utilities[`.color-vigi-${i}`] = {
          color: text,
          "background-color": bg,
          "border-color": text,
        };

        utilities[`.text-vigi-${i}`] = {
          color: text,
        };

        utilities[`.text-bg-vigi-${i}`] = {
          color: bg,
        };

        utilities[`.bg-vigi-${i}`] = {
          "background-color": bg,
        };

        utilities[`.bg-text-vigi-${i}`] = {
          "background-color": text,
        };
      }

      addUtilities(utilities);
    }),
  ],
};
