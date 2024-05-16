# connect4-rust

Run with `cargo`

```
cargo run
```

Or execute binary

```
target/release/connectfour
```

### For running in `docker`

#### Building Image

```docker
docker build -t connect4 .
```

#### Running container

```docker
docker run -i --rm -name connect4 connect4
```

`-i` : For running container in interactive mode

`--rm` : Removing container after finish
