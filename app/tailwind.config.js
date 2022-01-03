module.exports = {
  purge: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      maxWidth: {
        'xxs': '12rem',
      },
      colors: {
        none: '#333333',
        verb: '#ff6f6f',
        noun: '#70d0ff',
        adjective: '#70ff77',
        adverb: '#ffbb73',
        pronoun: '#cb85ff',
        conjunction: '#78a0ff',
        determiner: '#8b83f0',
        preposition: '#ffe387',
        other: '#545454',
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}