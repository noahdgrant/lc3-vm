.ORIG   x3000         ; Starting address
ADD     R0, R0, x3    ; Add number to register
ADD     R1, R1, X3    ; Add number to register
ADD     R2, R2, b0101 ; Add number to register
ADD     R3, R3, B0101 ; Add number to register
ADD     R4, R4, #7    ; Add number to register
ADD     R5, R0, R1    ; Add number to register
HALT                  ; Halt the program

.END                  ; End of program
