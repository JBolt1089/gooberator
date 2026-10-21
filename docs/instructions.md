# Instructions
This will contain a list of the instructions in gooberator

Format for instruction arguments: arg-name(arg-type)

Argument types should be fairly obvious as to what they are

## Printing instructions
```
prci - usage: prci char(u8) - function: prints char to the console
prcr - usage: prcr reg(register) - function: takes the lower 8 bits of reg, then interprets it as a char, then prints it
prab - usage: prab reg(register) - function: prints register reg as binary, useful for testing
prns - usage: prns target(u16) - function: prints chars stored in memory at target until it hits 0x0000 (terminator character)
```

## Arithmetic instructions
```
inc - usage: inc reg(register) - function: increments register reg by 1
dec - usage: dec reg(register) - function: decrements register reg by 1
add - usage: add input1(register) input2(register) output(register) - function: calculates input1 + input2 and stores the result in register output
sub - usage: sub input1(register) input2(register) output(register) - function: calculates input1 - input2 and stores the result in register output
```
