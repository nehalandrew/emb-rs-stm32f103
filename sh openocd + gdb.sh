#! /bin/bash

# Open On-Chip Debugger
openocd -f interface/stlink-v2-1.cfg -f target/stm32f1x.cfg &

# GNU DeBugger
gdb-multiarch -q -ex "target remote :3333" ./target/thumbv7m-none-eabi/debug/stm32;

load;

layout split;

monitor arm semihosting true;

continue;