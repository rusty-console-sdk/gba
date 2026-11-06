/* Linker script for a cartridge ROM. Shared by both backends. */

ENTRY(__start)

MEMORY
{
    rom   (rx)  : ORIGIN = 0x08000000, LENGTH = 32M
    /* The top 256 bytes of IWRAM hold the IRQ and SVC stacks and the interrupt
       vector; the user stack grows down from 0x03007F00. 2K below that is
       reserved for it, so an oversized .data or .bss fails at link time
       instead of being overwritten by the stack. */
    iwram (rwx) : ORIGIN = 0x03000000, LENGTH = 32K - 256 - 2K
    ewram (rwx) : ORIGIN = 0x02000000, LENGTH = 256K
}

SECTIONS
{
    .text :
    {
        KEEP(*(.text.header))
        *(.text .text.*)
        . = ALIGN(4);
    } > rom

    .rodata :
    {
        *(.rodata .rodata.*)
        . = ALIGN(4);
    } > rom

    /* Initialized data and code marked #[link_section = ".iwram"], copied
       from ROM by crt0. */
    .data :
    {
        . = ALIGN(4);
        __data_start = .;
        *(.iwram .iwram.*)
        *(.data .data.*)
        . = ALIGN(4);
        __data_end = .;
    } > iwram AT> rom
    __data_lma = LOADADDR(.data);

    .bss (NOLOAD) :
    {
        . = ALIGN(4);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        __bss_end = .;
    } > iwram

    __rom_end = LOADADDR(.data) + SIZEOF(.data);

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*)
        *(.ARM.extab .ARM.extab.*)
        *(.ARM.attributes)
        *(.comment)
        *(.note .note.*)
    }
}
