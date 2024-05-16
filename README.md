# connect4-rust
<img width="384" alt="Game Start" src="https://github.com/alasgarlikamal/connect4-rust/assets/98516464/a9c4b655-45b1-4c57-9d82-6e15125699ef">
<img width="383" alt="Game End" src="https://github.com/alasgarlikamal/connect4-rust/assets/98516464/098a40b3-4c5c-4b36-b374-fd05a98efb6a">

Run with `cargo`

```
cargo run
```

Or execute binary

```
cargo build --release
target/release/connectfour
```

### For running in `docker`

#### Building Image

```docker
docker build -t connect4 .
```

Image Size: `8.19 MB`

#### Running container

```docker
docker run -it --rm -name connect4 connect4
```

`-i` : For running container in interactive mode

`-t` : Allocating pseudo-TTY

`--rm` : Removing container after finish
