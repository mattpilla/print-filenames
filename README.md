# print-filenames
> print filenames matching glob pattern to textfile

```
$ print-filenames -h
print-filenames
print filenames matching glob pattern to textfile

USAGE:
    print-filenames.exe <PATTERN> -o <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o <FILE>        Sets the output file to write to

ARGS:
    <PATTERN>    Sets file pattern for input (use full windows path, ex: "D:\\user\\media\\**\\*.jpg")
```
Downloadable on releases page: https://github.com/mattpilla/print-filenames/releases

This is a basic Rust program to save filenames to a textfile with reasonable speed, since I wasn't able to find another tool with this functionality.

The filenames simply match whatever pattern you give it, so if you want absolute filepaths in your output file, your pattern should be on an absolute path as well. Any glob pattern should work.

This is actually cross-platform, but since this is just a simple script I didn't bother setting up multiple releases. All you have to do is download the source and build with `cargo build --release` on your target hardware, or specify the target with the `--target` flag. When running the program, you simply use the file path pattern your OS uses for the pattern argument.
