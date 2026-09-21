# rusty_blog

Static blog generator written in Rust. Reads Markdown posts from a content
directory and writes HTML into an output directory.

Requires **Rust 1.98+** (see `rust-toolchain.toml`).

## Build

```bash
cargo build --release
./target/release/rusty_blog /path/to/content /path/to/output
```

## Docker

Images are published to GHCR on every push to `main`:

`ghcr.io/shaneqi/rusty_blog:latest` and `ghcr.io/shaneqi/rusty_blog:<commit-sha>`

```bash
docker pull ghcr.io/shaneqi/rusty_blog:latest
docker build -t rusty_blog .
docker run --rm \
  -v /path/to/content:/content:ro \
  -v /path/to/site:/output \
  rusty_blog /content /output
```

Or use the helper script (defaults: sibling `../content` → this repo):

```bash
CONTENT=/path/to/content OUTPUT=/path/to/site ./docker/run.sh
```

Content layout:

```text
content/
  posts/
    hello-world.md          # front matter permalink: hello-world
    hello-world/            # sidecar folder named like the permalink
      hero.jpg
```

In Markdown, reference sidecar files relative to the published post page:

```markdown
![Hero](./hero.jpg)
```

Those files are copied to `output/<permalink>/` next to `index.html`.
