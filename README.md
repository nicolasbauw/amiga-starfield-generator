# Amiga starfield generator

This utility generates sprite data words to create a starfield.
It creates random stars on three "depths" (far, med, near) and a table with corresponding "speeds" (position increment).
You can choose the start and end lines, and the line interval between two stars:

```
Starting line (decimal - min = 44)?
149
Ending line (decimal - max = 298)?
254
Stars line interval (decimal)?
1
```

```
0x9520,0x9600,
0x8000,0x0000,
0x971F,0x9800,
0x8000,0x8000,
...
0xFD62,0xFE00,
0x8000,0x8000,
0x0000,0x0000

```

```
Speeds table:
[1, 3, 2, 1, 3, 1, 1, 1, 3, 3, 2, 2, 1, 1, 2, 3, 2, 1, 1, 1, 2, 2, 1, 2, 3, 1, 1, 3, 2, 2, 2, 2, 1, 3, 1, 1, 3, 3, 2, 3, 2, 2, 2, 1, 1, 1, 3, 1, 2, 1, 1, 2, 3]
53 stars generated.
```
It also handles vertical positions after $FF.  

To install:
```
cargo install amiga-starfield-generator
```