serve:
  dx serve --platform web

build: clean
  dx build --release --platform web
  cp -r target/dx/portfolio/release/web/public dist
  cp dist/index.html dist/404.html

clean:
  rm -rf dist
