# rusty_blog

Static blog generator written in Rust. Reads Markdown posts from a content
directory and writes HTML into an output directory.

## Docker

Build the image:

```bash
docker build -t rusty_blog .
```

Generate the site (mount your content and output directories):

```bash
docker run --rm \
  -v /path/to/content:/content:ro \
  -v /path/to/site:/output \
  rusty_blog /content /output
```

Or use the helper script (defaults: sibling `../content` → this repo):

```bash
CONTENT=/path/to/content OUTPUT=/path/to/site ./docker/run.sh
```

The image includes the binary plus `templates/` and `assets/`. Content is
expected to contain a `posts/` directory of Markdown files.
