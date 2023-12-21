/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./public/views/**/*.html"],
  theme: {
    extend: {
      colors: {
        "layer-one-light": "#e6e6f4",
        "layer-one-dark": "#090a0b",
      },
    },
  },
  // may add DaisyUI here later
  plugins: [require("@tailwindcss/forms")],
};
