# SMPL VM

This is a small project for the purpose of my learning of Rust. It implemets a runtime interpreter for the [SMPL language](https://github.com/adam-mcdaniel/smpl/), developed by Adam McDaniel. SMPL has eleven operations:

| Operation | Description                                                                      |
|-----------|----------------------------------------------------------------------------------|
| >         | pointer += 1                                                                     |
| <         | pointer -= 1                                                                     |
| +         | memory[pointer] += 1                                                             |
| -         | memory[pointer] -= 1                                                             |
| .         | print memory[pointer]                                                            |
| ,         | input -> memory[pointer]                                                         |
| [         | if memory[pointer] = 0<br>jump past matching ]                                   |
| ]         | if memory[pointer] != 0<br>jump to matching [                                    |
| *         | pointer = memory[pointer]                                                        |
| &         | Restore pointer to previous value before *                                       |
| ?         | memory[pointer] = address of first X cells set to 0<br>where X = memory[pointer] |

Adam McDaniel also created the [free language](https://github.com/adam-mcdaniel/free), which compiles from Rust-like syntax to SMPL operations. This emulator ought be capable of executing any such program. 