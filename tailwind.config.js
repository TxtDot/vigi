let bgColors = {};
let textColors = {};

for (let i = 0; i <= 100; i += 5) {
  bgColors[`vigi-${i}`] =
    `color-mix(in oklch, var(--max-bg) ${i}%, var(--min-bg))`;

  textColors[`vigi-${i}`] =
    `color-mix(in oklch, var(--max-text) ${i}%, var(--min-text))`;
}

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: bgColors,
      textColor: textColors,
    },
  },
  plugins: [],
};
