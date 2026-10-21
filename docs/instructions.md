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

## Memory and register instructions
```
li - usage: li reg(register) value(u16) - function: loads value into register reg
ldr - usage: ldr reg(register) target(u16) - function: loads the value at the offset target in memory into register reg
str - usage: str reg(register) target(u16) - function: stores the value in register reg in memory at the offset target
```

## Jump instructions
```
jmp - usage: jmp target(u16) - function: jumps execution to the instruction line target and pushes the return address onto the stack
jz - usage: jz reg(register) target(u16) - function: if the value of reg is zero, jumps execution to the instruction line target and pushes the return address onto the stack
jnz - usage: jz reg(register) target(u16) - function: if the value of reg is NOT zero, jumps execution to the instruction line target and pushes the return address onto the stack
ret - usage: ret - function: returns to the value on the stack and pops it. exits the program if there is nothing on the stack.
```

## Bit manipulation instructions
```
shl - usage: shl reg(register) amount(u16) - function: shifts the register reg left amount times
shr - usage: shr reg(register) amount(u16) - function: shifts the register reg right amount times
and - usage: and input1(register) input2(register) output(register) - function: does bitwise AND input1 & input2 and stores the result in output
or - usage: or input1(register) input2(register) output(register) - function: does bitwise OR input1 | input2 and stores the result in output
xor - usage: xor input1(register) input2(register) output(register) - function: does bitwise XOR input1 ^ input2 and stores the result in output
```

## Arithmetic instructions
```
inc - usage: inc reg(register) - function: increments register reg by 1
dec - usage: dec reg(register) - function: decrements register reg by 1
add - usage: add input1(register) input2(register) output(register) - function: calculates input1 + input2 and stores the result in register output
sub - usage: sub input1(register) input2(register) output(register) - function: calculates input1 - input2 and stores the result in register output
```
