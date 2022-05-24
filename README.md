# cpif

**cpif** is a command-line tool for copying files content to clipboard and providing metadata about the file.

# Usage

```bash
$ cpif [file] [options]
```

```bash
[options]:
    --help:
        Show help
    --copy:
        Copy to clipboard
    --info:
        Show all metadata
    --name:
        Show file name
    --type:
            Show file type
    --size:
            Show file size
    --created:
        Show file creation date
    --modified:
        Show file modification date
```

# Run `cpif` locally

To run cpif you will need rust and cargo installed.

```bash
$ cargo run ./cpif
```
