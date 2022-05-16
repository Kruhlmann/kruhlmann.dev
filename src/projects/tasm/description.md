Stack-based language compiling to statically-linked amd64 binaries.

TASM uses [reverse-polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) for its syntax to aid the stack based design. The compiler treats newlines as spaces.

# Examples

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

# Tokens

## `+`

Adds the 1st and 2nd items in the stack together and pushes the result back.

    1 2 +

Stack

    [1]
    [2,1]
    [3]

## `&`

Performs a bit-wise and on the top 2 elements and pushes the result

16 is 0x010 and 17 is 0x011

    16 17 &

Stack

    [16]
    [17,16]
    [1]

## `|`

Performs a bit-wise or on the top 2 elements and pushes the result

16 is 0x010 and 17 is 0x011

    16 17 |

Stack

    [16]
    [17,16]
    [17]

## `clone`

Pops the first element and pushes it back twice

    2 clone

Stack

    [2]
    [2,2]

## `clone2`

Pops the first two elements and pushes them back twice

    2 1 clone2

Stack

    [2]
    [1,2]
    [1,2,1,2]

## `--`

Pops the first element, decrements it, then pushes it back to the stack

    2 --

Stack

    [2]
    [1]

## `do`

Ends the conditional for a while loop

    1 while clone 0 > do clone put -- wend

Stack

    [1]
    [1,1]
    [0,1,1]
    [1]
    [1,1]
    [1]
    [0]

## `drop`

Pops the first element

    1 drop

Stack

    [1]
    []

## `else`

Negative conditional clause

    0 
    if
        "This is NOT executed\\n" 1 1 syscall3
    else
        "This IS executed\\n" 1 1 syscall3
    endif

    1
    unless
        "This is NOT executed\\n" 1 1 syscall3
    else
        "This IS executed\\n" 1 1 syscall3
    endif

Stack

    [1]
    []
    [1]
    []

## `=`

Pops the first two elements, compares them for equality and pushes the result (1 if equal, else 0)

    4 4 =

Stack

    [4]
    [4,4]
    [1]

## `>`


Pops the first two elements, checks if the first is greater than the second  and pushes the result (1 if a > b, else 0)

    4 5 >

Stack

    [4]
    [5,4]
    [0]

## `if`

Positive conditional clause

    1 
    if
        "This IS executed\\n" 1 1 syscall3
    else
        "This is NOT executed\\n" 1 1 syscall3
    endif

## `++`

Pops the first element, increments it, then pushes it back to the stack

    2 ++

Stack

    [2]
    [3]

## `load`

Pops the first element and uses it as a memory address, pushing the contents of the address to the stack

    mem load put

Stack
    
    [<memory_addr>]
    [<memory_addr>]
    []

## `<<`

Pops the first two elements, shifts a left by b and pushes the resutl back to the stack

    1 3 <<

Stack

    [1]
    [3,1]
    [8]

## `>`


Pops the first two elements, checks if the first is less than the second  and pushes the result (1 if a < b, else 0)

    4 5 <

Stack

    [4]
    [5,4]
    [1]

## `mem`

Pushes the first address in the available memory to the stack

    mem

Stack

    [<memory_addr>]

## `over`

Pops the first two elements, pushes back b, then a, then b

    1 2 over

Stack

    [1]
    [2,1]
    [1,2,1]

## `#number`

Push an integer to the stack

    4 5

Stack
    
    [4]
    [5,4]

## `#string`

Pushes the address of a string constant to the stack

    "Hello, World!"

Stack

    [<memory_addr>]

## `put`

Pops the first element and prints it to stdout

    4 put

Stack

    [4]
    []

## `>>`

Pops the first two elements, shifts a left by b and pushes the resutl back to the stack

    16 1 >>

Stack

    [16]
    [1,16]
    [8]

## `-`

Pops the first two elements, subtracts b from a and pushes the result back

    2 1 -

Stack

    [2]
    [1,2]
    [1]

## `swap`

Swap the first and second element

    2 1 swap

Stack

    [2]
    [1,2]
    [2,1]

## `syscall1`

Performs a syscall with 1 argument from the stack. Example calls the exit syscall (60) with a single argument (80) and exits with exit code 80

    80
    60 syscall1

Stack

    [80]
    [60,80]
    []

## `syscall3`

Performs a syscall with 3 arguments from the stack (prints abc).

    mem 0 + 97 write
    mem 1 + 98 write
    mem 2 + 99 write
    3 mem 1 1 syscall3

Stack

    [<memory_addr>]
    [0,<memory_addr>]
    [<memory_addr>]
    [97,<memory_addr>]
    []
    [<memory_addr>]
    [1,<memory_addr>]
    [<memory_addr>]
    [98,<memory_addr>]
    []
    [<memory_addr>]
    [2,<memory_addr>]
    [<memory_addr>]
    [99,<memory_addr>]
    []
    [3]
    [<memory_addr>,3]
    [1,<memory_addr>,3]
    [1,1,<memory_addr>,3]
    []

## `unless`

Inverse positive conditional clause

    0
    unless
        "This IS executed\\n" 1 1 syscall3
    else
        "This is NOT executed\\n" 1 1 syscall3
    endif

## `wend`

Ends the execution block for a while loop

    1 while clone 0 > do clone put -- wend

Stack

    [1]
    [1,1]
    [0,1,1]
    [1]
    [1,1]
    [1]
    [0]

## `while`

Starts the conditional block for a while loop

    1 while clone 0 > do clone put -- wend

Stack

    [1]
    [1,1]
    [0,1,1]
    [1]
    [1,1]
    [1]
    [0]

## `write`

Pops the first two elements and writes b to the address in memory a

    mem 97 write

Stack

    [<memory_addr>]
    [97,<memory_addr>]
    []

