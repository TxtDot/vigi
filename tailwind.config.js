/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        main: "var(--c0)",
        block: "var(--c1)",
        light: "var(--c2)",
        dark: "var(--c3)",
      },
      textColor: {
        main: "var(--c2)",
      },
    },
  },
  plugins: [],
};
