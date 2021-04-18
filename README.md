# hours

`hours` is a Rust program for tracking hours worked and making billing easy.

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
