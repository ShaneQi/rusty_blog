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

Content layout (page bundles):

```text
content/
  posts/
    hello-world/
      hello-world.md   # required: <permalink>/<permalink>.md
      hero.jpg
      nested/chart.png
```

The folder name is the permalink. In Markdown use paths relative to that folder
(they work both when previewing the `.md` and on the published page):

```markdown
![Hero](./hero.jpg)
```

Assets (everything except `.md` files) are copied to `output/<permalink>/`
next to `index.html`.
