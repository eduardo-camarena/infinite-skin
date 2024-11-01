/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./src/**/*.{html,js,jsx,ts,tsx}', './index.html'],
  theme: {
    screens: {
      sm: '320px',
      md: '768px',
      lg: '1024px',
      xl: '1280px',
    },
    extend: {
      transitionProperty: {
        width: 'width',
        height: 'height',
      },
    },
  },
  plugins: [require('@tailwindcss/forms')],
};
