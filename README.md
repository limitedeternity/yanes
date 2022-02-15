# yanes

> The most hardcore calculator you’ve ever seen

Yanes is a software-based device model (simulator), which implements 6502-like CPU instructions, memory address space, a screen and a keyboard.

## Spec

**OpCodes**: http://www.6502.org/tutorials/6502opcodes.html

**Address Space Layout**:

- CPU Data: **0x0000 – 0x00FF**
  
  - The latest keypress in form of ASCII code is stored at **0x00FF**.

- Stack: **0x0100 – 0x01FF**

- VGA Buffer: **0x0200 – 0x05FF**
  
  * Layout:
    
    ![image](https://user-images.githubusercontent.com/24318966/104934963-b4158e00-59bb-11eb-8add-fcc913b8bc83.png)
  
  * Color Codes:
    
    ![yanes](https://user-images.githubusercontent.com/24318966/104935109-e2936900-59bb-11eb-8bb3-b2754d017906.png)

- Program Space: **0x8000 – 0xFFF0**
  
  - Therefore, max size is **0x7FF0** bytes.
  - Programs are executable machine code sequences for this simulator. You can either write one manually using a hex editor of your choice or use an assembler to produce it.

- Reset Vector: **0xFFFC**

- IRQ Vector: **0xFFFE**
  
  - If you want to override `BRK` behavior, you can write an address of your custom procedure there. 
    In case you want to revert its default behavior (program termination), you just need to null the IRQ Vector.

## Example

Let’s look into the `example/` directory:

![1](https://user-images.githubusercontent.com/24318966/105477350-cbea5c00-5cb2-11eb-82f3-553d049148be.png)

This program draws 0 or 1 depending on whether `0` or `1` was pressed on the keyboard. 
Let’s compile our example assembly script using my fork of [@jenra-uwu's asm6502](https://github.com/jenra-uwu/asm6502): `asm6502/target/release/asm6502 -o examples/draw_0_or_1.bin -d examples/draw_0_or_1.s`.
After that we can run the compiled program using yanes: `target/release/yanes examples/draw_0_or_1.bin`.

![loop](https://user-images.githubusercontent.com/24318966/105478172-e6710500-5cb3-11eb-9900-e1a466a9469f.png)

We see, that infinite loop works correctly. Now, if we press `0`:

![0](https://user-images.githubusercontent.com/24318966/105478252-06082d80-5cb4-11eb-803c-dd8bcb281ffa.png)

We'll see a drawn zero. And if we press `1`:

![1](https://user-images.githubusercontent.com/24318966/105478325-1a4c2a80-5cb4-11eb-9779-d2f64e723221.png)

The screen will be cleared and a drawn 1 will appear. To exit Yanes, press `ESC`. 

`ESC` key injects `BRK` instruction at the `PC` register position and, therefore, terminates the program. After that, a CPU dump will be printed into the `STDOUT` in form of state of all registers at the moment of program termination:

![dump](https://user-images.githubusercontent.com/24318966/105478702-90e92800-5cb4-11eb-971f-8cbdb549c684.png)

## Building

1. Install the Rust toolchain using [rustup.rs](https://rustup.rs/).
2. `git clone --recurse-submodules` this repo.
3. Pull your favourite C/C++ compiler ([Windows](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019), [Linux](https://en.wikipedia.org/wiki/You_Know_What_to_Do), [MacOS](https://www.ics.uci.edu/~pattis/common/handouts/macmingweclipse/allexperimental/macxcodecommandlinetools.html)).
4. Run `cargo build --release` there and in `asm6502/` directory.

## Meta

Vyacheslav Bespalov  – [Other projects](https://github.com/limitedeternity?tab=repositories)

Distributed under the MIT license. See `LICENSE` for more information.

[@limitedeternity](https://github.com/limitedeternity)

## Contributing

1. [Fork it](https://github.com/limitedeternity/yanes/fork)
2. Commit your changes
3. Create a new Pull Request
