.ORIG   x3000         ; Starting address
ADD     R0, R0, x3    ; Tests x and adding number to regsiter
ADD     R1, R1, X3    ; Tests X
ADD     R2, R2, b0101 ; Tests b
ADD     R3, R3, B0101 ; Tests B
ADD     R4, R4, #7    ; Tests #
ADD     R5, R0, R1    ; Tests adding register to register
HALT                  ; Halt the program

.END                  ; End of program
