/* 
https://tech.microbit.org/hardware/#hardware-description 
https://docs.nordicsemi.com/bundle/nRF52833_PS_v1.6/resource/nRF52833_PS_v1.6.pdf | page 21
*/

MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K     
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K    
}