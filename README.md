# hours

`hours` is a Rust program for tracking hours worked and making billing easy.

### Building

You can use `cargo` to build this project:
```bash
cargo build --release
```
The output binary ends up in `./target/release/hours`.

There is also a `Makefile`. Running `make install` will build the project and copy the output binary to `~/bin/`.

### Configuration

You need to create a `~/.config/hours.yaml` file with the following contents:
```yaml
api_key: Github API key
gist_id: Githb Gist ID, if you're starting this should be an empty gist
```

You can generate an API key [here](https://github.com/settings/tokens).
When creating a gist, you can't make it empty. You have to provide some content first, and then after creating it
you can edit it to be empty.

### How it works

Locally, hours worked are stored at `~/hours.txt`. When starting or exiting the program, the local hours will be synced
with the gist.
