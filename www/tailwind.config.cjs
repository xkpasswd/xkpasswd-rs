/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{ts,tsx}"
  ],
  theme: {
    extend: {},
    colors: {
      transparent: 'transparent',
      current: 'currentColor',
      black: '#282a36',
      red: '#ff5c57',
      blue: '#57c7ff',
      white: '#eff0eb',
    }
  },
  plugins: [],
}
