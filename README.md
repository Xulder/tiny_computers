# CPU Emulator

A CPU emulator framework written in Rust, focusing on accurate register and instruction emulation with a modular architecture. This was originally designed to be JUST a library for a game of mine. Then it grew... So here it is, I guess.

## Overview

This project provides a comprehensive CPU emulation framework that simulates processor behavior through:

- Register operations and state management
- Flexible instruction set architecture (ISA)
- Memory management with device mapping
  Error handling and debugging capabilities

## Features

### Core CPU Components

- Complete CPU state management
- Flexible register file implementation
- Flag register support
- Cycle-accurate execution

### Memory System

- Modular memory device architecture
- Memory mapping capabilities
- Support for ROM and RAM devices
- Memory bus implementation for device management

### Instruction Set Architecture

- Extensible instruction set framework
- Multiple addressing modes
- Instruction encoding/decoding
- Support for various instruction categories:
  - Arithmetic
  - Logic
  - Data Transfer
  - Control Flow
  - Stack Operations
  - I/O Operations
  - System Instructions

## Plans

The `core` module is the result of my over-engineering tick. It's a mess, and as of 0.1.1, I haven't even finished an implementation to see if it works. I plan for the `arch` module to be the next big thing—it's going to be a modular architecture that allows for easy addition of new architectures. I will proceed to over-engineer the `arch` module, still not knowing if `core`—the very module that `arch` is based on—works. I mean, I'm not even sure if I'm going to finish `core` before I start working on `arch`. I started a little bit on a CHIP-8 emulator... with like one file...

- [ ] Core module
  - [ ] Unit tests
  - [ ] Documentation
  - [ ] Debugging
  - [ ] Logging
  - [ ] Probably peripheral devices eventually
- [ ] Arch module
  - [ ] CHIP-8 (It just seemed like a good start)
  - [ ] Z80 (I just think it's neat)
  - [ ] ZTiny8 and ZTiny16 (My own architecture, which will be used in my game)
  - [ ] GameBoy (I feel like writing a GameBoy emulator is a rite of passage for any self-respecting programmer—(I do not respect myself, as I have not done this yet))
  - [ ] PIC16F84 (I just think it's neat)
  - [ ] 709 (Vacuum tubes)
