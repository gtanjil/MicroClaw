/* memory.x - Defines the memory layout for MicroClaw */
MEMORY
{
  /* ROM (Flash) - Where the code lives */
  FLASH : ORIGIN = 0x42000000, LENGTH = 4M
  
  /* RAM - Where the variables live */
  RAM   : ORIGIN = 0x40380000, LENGTH = 400K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_STACK", RAM);