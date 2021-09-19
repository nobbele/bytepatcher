# bytepatcher
Patches bytes in binary files

```
$ bytepatcher --help
BytePatcher 1.0
nobbele <realnobbele@gmail.com>
Patches bytes in binaries

USAGE:
    bytepatcher <file> -s <signature> -p <patch>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --signature <signature>    Signature to find the bytes to patch.
    -p, --patch <patch>            Bytes to replace signature with.

ARGS:
    <file>    File to patched bytes in.
```
Example usage of patching 2012 osu! to use 127.0.0.1 for bancho
```
$ bytepatcher osu!.exe -s 32174A5D -p 7F000001
Signature: [32, 17, 4A, 5D]
Patch: [32, 17, 4A, 5D]
Found signature at 119C1E!
Patched file has been writted to osu!-patched.exe
```
