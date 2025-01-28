/** @type {import('tailwindcss').Config} */
export default {
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  content: [],
  theme: {
    extend: {
      backgroundImage: {
        'diagonal-stripes': `linear-gradient(
          45deg,
          #ffcfdf 25%,
          transparent 25%,
          transparent 50%,
          #ffcfdf 50%,
          #ffcfdf 75%,
          transparent 75%,
          transparent
        )`
      },
      backgroundSize: {
        'stripes': '20px 20px'
      }
    },
  },
  plugins: [],
}

