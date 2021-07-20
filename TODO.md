# Features

## More settings

- FnInput
  - Handle points
- Toggle
  - display

# Hosting

## On GitHub

To host a WASM app in GitHub, do the following:

- Build the release with trunk

  ```shell
  trunk build --release
  ```

- Copy from `dist` folder to `docs` folder to update github-pages.

- Change the path of the `.js` and `.wasm` files.
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

