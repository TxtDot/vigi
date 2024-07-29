let colors = {};

for (let i = 0; i <= 100; i += 5) {
  colors[`vigi-${i}`] = `color-mix(in hsl, var(--max) ${i}%, var(--min))`;
}

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors,
      textColor: colors,
    },
  },
  plugins: [],
};
