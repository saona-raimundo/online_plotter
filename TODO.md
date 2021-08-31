- Change default (no curly bracket needed)
- Check restoring from storeload

# Library

## app -> library?

### Canvas

- Check out [yew-components](https://crates.io/crates/yew-components/0.3.0)
- Create yew_plotters crate

### Testing

- use [wasm-bindgen-test](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/index.html)

# Features

## Website

- FAQ
  - Static

## Frontend

### Resize

- https://stackoverflow.com/questions/8960193/how-to-make-html-element-resizable-using-pure-javascript
- http://anhr.github.io/resizer/
- http://jsfiddle.net/3jMQD/
- https://www.w3schools.com/cssref/css3_pr_resize.asp
- https://spin.atomicobject.com/2019/11/21/creating-a-resizable-html-element/

### Responsive design

- Create two (or more) different views!
- https://www.w3schools.com/css/css_rwd_viewport.asp

## Backend



# Hosting (info, not to do)

## On GitHub

To host a WASM app in GitHub, do the following:

- Build the release with trunk

  ```shell
  trunk build --release
  ```

- Copy from `dist` folder to `docs` folder to update github-pages.

- In `index.html`, <link> and <body> script, change the path of the `.js` and `.wasm` files.
  From

  ```
  import init from '/index-e0537d81cb66b4d0.js';
  init('/index-e0537d81cb66b4d0_bg.wasm');
  ```

  To 

  ```
  import init from './index-e0537d81cb66b4d0.js';
  init('./index-e0537d81cb66b4d0_bg.wasm');
  ```

  (adds a dot `.`)
  
- Check with `miniserve`.

