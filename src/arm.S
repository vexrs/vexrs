.text
.arm

; Different system modes
.set SYS_MODE, 0x1f
.set SVC_MODE, 0x13
.set IRQ_MODE, 0x12

; Hardware registers
.extern ulICCIAR
.extern ulICCEOIR
.extern ulICCPMR

IRQ_Handler:
    /* Return to the interrupted instruction */
    SUB lr, lr, #4

    /* Push the return address and SPSR */
    PUSH {lr}
    MRS lr, SPSR
    PUSH {lr}

    /* Push the used registers */
    PUSH {r0-r4, r12}

    