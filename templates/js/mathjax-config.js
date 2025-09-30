window.MathJax = {
  tex: { inlineMath: [['$', '$'], ['\\(', '\\)']], displayMath: [['$$','$$'], ['\\[','\\]']] },
  loader: {
    paths: { font: 'https://cdn.jsdelivr.net/npm/@mathjax' },
    load: ['[font]/mathjax-euler-font']
  }
};