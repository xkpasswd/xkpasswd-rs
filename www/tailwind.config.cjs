/** @type {import('tailwindcss').Config} */
// Snazzy as a *dark* theme. black/white swap roles vs the old config.
// surface/overlay/muted/faint/stdout are DERIVED (not in Snazzy) — tuned for dark.
module.exports = {
  content: ['./index.html', './src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      fontFamily: { mono: ['"Source Code Pro"','ui-monospace','SFMono-Regular','Menlo','monospace'] },
      keyframes: { blink: { '50%': { opacity: '0' } } },
      animation: { blink: 'blink 1.1s steps(1) infinite' },
    },
    colors: {
      transparent: 'transparent', current: 'currentColor',
      black: '#282a36', white: '#eff0eb',
      red: '#ff5c57', green: '#5af78e', yellow: '#f3f99d',
      blue: '#57c7ff', magenta: '#ff6ac1', cyan: '#9aedfe',
      surface: '#30323e', overlay: '#3a3d4a', stdout: '#23252f',
      muted: '#888da0', faint: '#5d6173',
      gray: '#888da0', // alias for any leftover text-gray/*
    },
  },
  plugins: [],
};
