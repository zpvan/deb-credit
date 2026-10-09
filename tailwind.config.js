/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ['./index.html', './src/**/*.rs'],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: 'rgb(var(--primary) / <alpha-value>)',
          container: 'rgb(var(--primary-container) / <alpha-value>)',
        },
        'on-primary': 'rgb(var(--on-primary) / <alpha-value>)',
        'on-primary-container': 'rgb(var(--on-primary-container) / <alpha-value>)',
        'secondary-container': 'rgb(var(--secondary-container) / <alpha-value>)',
        'on-secondary-container': 'rgb(var(--on-secondary-container) / <alpha-value>)',
        earn: {
          DEFAULT: 'rgb(var(--earn) / <alpha-value>)',
          container: 'rgb(var(--earn-container) / <alpha-value>)',
        },
        'on-earn': 'rgb(var(--on-earn) / <alpha-value>)',
        'on-earn-container': 'rgb(var(--on-earn-container) / <alpha-value>)',
        surface: 'rgb(var(--surface) / <alpha-value>)',
        'surface-container-lowest': 'rgb(var(--surface-container-lowest) / <alpha-value>)',
        'surface-container-low': 'rgb(var(--surface-container-low) / <alpha-value>)',
        'surface-container-high': 'rgb(var(--surface-container-high) / <alpha-value>)',
        'surface-container-highest': 'rgb(var(--surface-container-highest) / <alpha-value>)',
        'on-surface': 'rgb(var(--on-surface) / <alpha-value>)',
        'on-surface-variant': 'rgb(var(--on-surface-variant) / <alpha-value>)',
        outline: 'rgb(var(--outline) / <alpha-value>)',
        'outline-variant': 'rgb(var(--outline-variant) / <alpha-value>)',
        error: {
          DEFAULT: 'rgb(var(--error) / <alpha-value>)',
          container: 'rgb(var(--error-container) / <alpha-value>)',
        },
        'on-error': 'rgb(var(--on-error) / <alpha-value>)',
        'on-error-container': 'rgb(var(--on-error-container) / <alpha-value>)',
      },
      fontFamily: {
        display: ["'Baloo 2'", "'PingFang SC'", "'Microsoft YaHei'", 'sans-serif'],
      },
      boxShadow: {
        'elevation-1': '0 1px 2px rgb(60 32 0 / 0.10), 0 1px 3px rgb(60 32 0 / 0.06)',
        'elevation-2': '0 2px 6px rgb(60 32 0 / 0.12), 0 1px 3px rgb(60 32 0 / 0.08)',
        'elevation-3': '0 4px 12px rgb(60 32 0 / 0.16)',
      },
    },
  },
}
