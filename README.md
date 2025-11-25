# Personal website

A simple static site generator with Rust for my personal website. It includes some blogs that written in Djot, then render and insert to an HTML template, and finally host it to Cloudflare with GitHub Action. Previously was Next.JS, Markdown, and Vercel.  

## A few tiny tweaks

- Djot doesn’t natively support metadata, but we can include it by starting a fenced block with a marker like `=toml`, which lets the block be treated and parsed as TOML. 

- Fonts are subsetted during the CI/CD to include only the required glyphs, reducing each WOFF2 file's size by roughly 63% and resulting in faster page loads and lower bandwidth usage. 
