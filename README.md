# brainfork

Optimizing bytecode VM for Brainfuck

Compilation is performed in a single pass

## `-O0`

Each source instruction is compiled to one bytecode instruction

## `-O1`

Repeated sequences of `+`/`-`/`>`/`<` are coalesced

## `-O2`

Small loops that perform common operations are optimized:

- `[+]`/`[-]`
- `[>]`
- `[<]`
- `[>+<-]`/`[->+<]`
- `[<+>-]`/`[-<+>]`
