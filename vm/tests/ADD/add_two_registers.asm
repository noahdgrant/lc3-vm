        .ORIG   x3000      ; Starting address
        LD      R1, NUM1   ; Load the first number into R1
        LD      R2, NUM2   ; Load the second number into R2
        ADD     R3, R1, R2 ; Add R1 and R2, store the result in R3
        ST      R3, RESULT ; Store the result in memory
        HALT               ; Halt the program

NUM1    .FILL   x0005      ; First number (5)
NUM2    .FILL   x0003      ; Second number (3)
RESULT  .BLKW   1          ; Reserve one memory location for the result
        .END               ; End of program
