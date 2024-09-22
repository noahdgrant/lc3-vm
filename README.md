# LC-3 Implementation in Rust

## Resources
https://www.jmeiners.com/lc3-vm/

https://www.rodrigoaraujo.me/posts/lets-build-an-lc-3-virtual-machine/

https://github.com/phy1um/rust-simple-vm/blob/main/vm/src/bin/vm.rs

Hello world for LC-3
```asm
.ORIG x3000                        ; this is the address in memory where the program will be loaded
LEA R0, HELLO_STR                  ; load the address of the HELLO_STR string into R0
PUTs                               ; output the string pointed to by R0 to the console
HALT                               ; halt the program
HELLO_STR .STRINGZ "Hello World!"  ; store this string here in the program
.END                               ; mark the end of the file
```

Use xxd to see binary output from file for looking at VM code
