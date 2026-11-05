@ ROM header and startup code. Assembled by `just asm` with arm-none-eabi-as
@ and linked into every ROM on both backends through gba.ld.
@
@ The BIOS jumps to the start of ROM in ARM mode with interrupts enabled in
@ CPSR, IME clear, and the stacks already set. All three stacks are set again
@ here so the layout doesn't depend on the BIOS.

    .syntax unified
    .cpu arm7tdmi
    .arm

    .section .text.header, "ax"
    .global __start
__start:
    b       entry
    @ 0x04..0xBF: Nintendo logo, title, game code, maker code, fixed byte,
    @ version and checksum. gbafix fills these in after the link.
    .space  0xBC

    .arm
entry:
    @ IRQ and SVC (BIOS call) stacks, then system mode with IRQs still
    @ enabled in CPSR (IME masks them until the SDK turns them on).
    mov     r0, #0x12
    msr     cpsr_c, r0
    ldr     sp, =0x03007FA0
    mov     r0, #0x13
    msr     cpsr_c, r0
    ldr     sp, =0x03007FE0
    mov     r0, #0x1F
    msr     cpsr_c, r0
    ldr     sp, =0x03007F00

    @ Copy .data (which includes code marked for IWRAM) from ROM to IWRAM.
    ldr     r0, =__data_lma
    ldr     r1, =__data_start
    ldr     r2, =__data_end
    bl      copy_words

    @ Zero .bss.
    ldr     r1, =__bss_start
    ldr     r2, =__bss_end
    bl      zero_words

    @ Into Rust. `main` is Thumb, and the linker sets bit 0 of its address.
    ldr     r0, =main
    bx      r0

@ r0: source, r1: destination, r2: end of destination. All word aligned.
    .arm
copy_words:
    cmp     r1, r2
    ldrlo   r3, [r0], #4
    strlo   r3, [r1], #4
    blo     copy_words
    bx      lr

@ r1: destination, r2: end of destination. Word aligned.
    .arm
zero_words:
    mov     r3, #0
1:
    cmp     r1, r2
    strlo   r3, [r1], #4
    blo     1b
    bx      lr

    .pool
