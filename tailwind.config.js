const plugin = require("tailwindcss/plugin");

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: ["./{src,assets}/**/*.{js,rs}"],
  theme: {
    extend: {
      theme: { colors: { cvation: "#002543" } },
      animation: {
        shake: "shake 0.82s cubic-bezier(.36,.07,.19,.97) both",
      },
      keyframes: {
        shake: {
          "10%, 90%": {
            transform: "translate3d(0, -1px, 0)",
          },
          "20%, 80%": {
            transform: "translate3d(0, 1px,  0)",
          },
          "30%, 50%, 70%": {
            transform: "translate3d(0, -1px,  0)",
          },
          "40%, 60%": {
            transform: "translate3d(0, 2px,  0)",
          },
        },
      },
    },
  },
  future: {
    hoverOnlyWhenSupported: true,
  },
  plugins: [
    plugin(function ({ addVariant }) {
      addVariant("htmx-settling", ["&.htmx-settling", ".htmx-settling &"]);
      addVariant("htmx-request", ["&.htmx-request", ".htmx-request &"]);
      addVariant("htmx-swapping", ["&.htmx-swapping", ".htmx-swapping &"]);
      addVariant("htmx-added", ["&.htmx-added", ".htmx-added &"]);
    }),
  ],
};
