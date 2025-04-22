# img-colorizer
Tool to help convert any image to use a given color palette. This works best on images that use flat colors, and doesn't do so well on things like photographs.

## Compiling
```
cargo build -r
```
The binary can then be found at `target/release/img-colorizer`.

## Usage
```
img-colorizer <colors> <image>
```
For example, `img-colorizer colors.txt image.png` with `colors.txt` looking something like this:

```
#ffff00
#ff00ff
#4d70ff
#70ff4d
#111111
#405040
#787080
#e0e0e0
```

**Note:** when this binary runs, it will generate (and possibly overwrite) a file at `output.png`.
