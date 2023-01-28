# About

This is my implementation of [PNGme](https://picklenerd.github.io/pngme_book/introduction.html).

This is an interesting project, and I might add some more features to it soon.

# External dependencies

[crc](https://crates.io/crates/crc) (3.0.0) and [clap](https://github.com/clap-rs/clap) (4.0.0)

# Usage

## Help

```Shell
cargo run -- --help
```

## Encode

```Shell
cargo run -- encode <path_to_source_png_file> <chunk_type> <message> [path_to_dest_png_file]
```

Where the `chunk_type` is all 4 byte length word, like `"rust"`, and `path_to_dest_png_file` is optional. If not provided, it will be the same with `path_to_source_png_file`.

The program will append a new chunk to the PNG file's end with patter `chunk_type`, containing the `message` in the chunk's `data` field.

## Decode

```Shell
cargo run -- decode <path_to_source_png_file> <chunk_type>
```

The program will search the first chunk that matches `chunk_type` pattern and print its data.

## Remove

```Shell
cargo run -- remove <path_to_source_png_file> <chunk_type>
```

The program will search the first chunk that matches `chunk_type` pattern and remove that chunk.

## Print

```Shell
cargo run -- print <path_to_source_png_file>
```

The program will print all chunks' type and length of the given PNG file.

# Example

Suppose I have an test.png at `./images/`.

```Shell
cargo run -- encode ./images/test.png "rust" "This is a secret message!" ./images/out.png
cargo run -- print ./images/test.png
cargo run -- decode ./images/out.png "rust"
cargo run -- remove ./images/out.png "rust"
```