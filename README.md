# connect4-rust

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
