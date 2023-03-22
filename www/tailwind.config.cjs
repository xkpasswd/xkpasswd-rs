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
      blue: '#57c7ff',
      gray: '#686868',
      magenta: '#ff6ac1',
      red: '#ff5c57',
      white: '#eff0eb',
    }
  },
  plugins: [],
}
