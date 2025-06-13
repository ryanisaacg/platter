# platter

A simple utility to serve you files on a `platter`

`platter` works on both desktop and web, and returns a byte buffer of the file's contents.
On desktop, `load_file` is backed by native file system APIs. On web, it is backed by an
HTTP 'GET' request.

```rust
let file_contents = load_file("path_to_my_file").await?;
```

