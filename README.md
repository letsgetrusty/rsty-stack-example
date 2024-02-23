# RSTY Stack Example App

Corresponding video: https://youtu.be/luOgEhLE2sg

- Suggest you install dependencies first:
  ```bash
  cargo install --locked wasm-bindgen-cli
  cargo install --locked trunk
  cargo install cargo-watch
  cargo install tauri-cli
  ```

- install Postman.

Run the following commands in separate terminals:

todo_api
```
    cargo watch -c -q -x run
```

todo_web
```
    trunk serve --open --port 8081
```

todo_desktop
```
    cargo tauri dev
```
