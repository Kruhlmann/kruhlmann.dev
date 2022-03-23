Stack-based language compiling to statically-linked amd64 binaries.

TASM uses [reverse-polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) for its syntax to aid the stack based design. The compiler treats newlines as spaces.

    1 2 + put

Print `3` to `stdout`

    0 if 2 put endif
    0 unless 4 put endif

Print `4` to `stdout`

    10 # Add 10 to the stack
    while clone 0 > do # Start while loop, every iteration
                       # clone the first item in the stack.
                       # Add 0 to the stack. Continue if the
                       # second item (10, 9, 8 ...) is greater
                       # than the next item, 0. If not go to wend.
        clone # Push the first item on the stack again
        put # Remove the first item from the stack and print it
        -- # Decrement the value of the first time
           # (10->9 in first iteration)
    wend
    11 # Add 11 to the stack
    put # Remove the first item from the stack and print it  (11)

Minified

    10 while clone 0 > do clone put -- wend 11 put

Print the numbers from `10` to `1` (inclusive) and then `11` to `stdout`

      mem   # Push the first memory address to the stack
      0     # Push 0 to the stack
      +     # Add the two first items together (memory offset 0)
      97    # Push 97 ('a' in ASCII) to the stack.
      write # Write the first item to the second item as a memory address

      mem 1 + 98 write # Write 'b' to memory + 1 byte offset
      mem 2 + 99 write # Write 'c' to memory + 2 byte offset
      3   # Push 3 to the stack (We need to print 3 characters)
      mem # Push the first memory address to the stack (start of string)
      1   # Push 1 to the stack
      1   # Push 1 to the stack (1 is sys_write which writes to stdout)
      syscall3 # Perform a syscall with 3 arguments
      44 put # Push 44 to the stack and print it

      80 # Push 80 to the stack (this will be the exit code)
      60 # Push 60 to the stack (60 is sys_exit_
      syscall1 # Perform a syscall with 1 argument (the exit code 80)

Minified

    mem 0 + 97 write mem 1 + 98 write mem 2 + 99 write 3 mem 1 1 syscall3 44 put 80 60 syscall1

Prints `"abc44"` to `stdout` and exits with exit code `80`
