Stack-based language compiling to statically-linked amd64 binaries.

TASM uses [reverse-polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) for its syntax to aid the stack based design.

`1 2 + put`

Print `3` to `stdout`

`0 if 2 put endif 0 unless 4 put endif`

Print `4` to `stdout`

`10 while cone 0 > do clone put -- wend 11 put`

Print the numbers from `10` to `1` (inclusive) and then `11` to `stdout`

`mem 0 + 97 write mem 1 + 98 write mem 2 + 99 write 3 mem 1 1 syscall3 44 put 80 60 syscall1`

Prints `"abc44"` to `stdout`
