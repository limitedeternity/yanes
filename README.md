# yanes
> The most hardcode calculator you’ve ever seen

Yanes is a software-based device model (emulator), which implements 6502-like CPU instructions, memory address space, a screen and a keyboard.

## Spec

**OpCodes**: http://www.6502.org/tutorials/6502opcodes.html

**Address Space Layout**:

- CPU Data: **0x0000 – 0x00FF**

  - The latest keypress in form of ASCII code is stored in **0x00FF**.

- Stack: **0x0100 – 0x01FF**

- VGA Buffer: **0x0200 – 0x0600**

  * Layout:

    ![image](https://user-images.githubusercontent.com/24318966/104934963-b4158e00-59bb-11eb-8add-fcc913b8bc83.png)

  * Color Codes:

    ![yanes](https://user-images.githubusercontent.com/24318966/104935109-e2936900-59bb-11eb-8bb3-b2754d017906.png)

- Program Space: **0x8000 – 0xFFFD**

  - Therefore, max size is **0x7FFD** bytes.
  - Programs are an executable bytecode for this emulator. You can write it manually using a hex editor of your choice or use an assembler to produce it.

## Example

![Yanes 2021-01-18 18-42-53](https://user-images.githubusercontent.com/24318966/104936066-17ec8680-59bd-11eb-898e-139b4051bc86.png)

`ESC` key injects `BRK` instruction at the `PC` register position and, therefore, terminates the program. 

After that, a CPU dump is printed into the `STDOUT` in form of state of all registers at the moment of program termination.

![yanes-1](https://user-images.githubusercontent.com/24318966/104936922-3c952e00-59be-11eb-9596-b7763beaa776.png)

## Building

1. Install the Rust toolchain using [rustup.rs](https://rustup.rs/).
2. `git clone` this repo.
3. Follow the [instructions](https://github.com/Rust-SDL2/rust-sdl2) to build SDL2.
4. Run `cargo build --release` there.

## Meta

Marise Hayashi – [Other projects](https://limitedeternity.github.io/)

Distributed under the MIT license. See ``LICENSE`` for more information.

[@limitedeternity](https://github.com/limitedeternity)

## Contributing

1. [Fork it](https://github.com/limitedeternity/yanes/fork)
2. Commit your changes
3. Create a new Pull Request
