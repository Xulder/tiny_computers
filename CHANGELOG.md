## Unreleased

### Feat

- **core/debug**: retiring debug system for now

### Fix

- refreshed CHANGELOG.md

### Refactor

- **core**: rename Value to Word across all core modules
- **isa**: streamline trait hierarchy and improve type naming

## v0.8.0 (2024-12-24)

### Fix

- refreshed CHANGELOG.md
- refreshed CHANGELOG.md

## v0.7.0 (2024-12-24)

### Feat

- **core/debug**: retiring debug system for now
- **core/debug**: retiring debug system for now
- **core/debug**: retiring debug system for now
- **core/debug**: seperating out debug functionality

### Refactor

- **core**: rename Value to Word across all core modules
- **isa**: streamline trait hierarchy and improve type naming

## v0.6.0 (2024-12-24)

### Feat

- **src**: modularization

### Refactor

- **src**: general cleanup and documentation

## v0.5.0 (2024-12-24)

### Feat

- **isa**: basic opcode and instruction encoding scheme
- **src**: first draft
- **src**: Full restart
- **cpu+isa**: first draft of `isa` module + `cpu` module cleanup
- **ROM + RAM**: removed the `MemoryDevice` trait

### Refactor

- **memory_mapper**: moved `memory_mapper.rs` to it's own module

## v0.4.0 (2024-12-24)

### Feat

- **memory+mapper**: working memory mapper
- **memory**: large redesign to memory
- **opcode.rs**: from u8 and Display for Mode and Instruction

### Refactor

- **restructure**: A little more restructuring for clarity
- file restructure
- **modularization**: cleanup and structure refactor
- **modularization**: cleanup and structure refactor
- **modularization**: broke down existing modules into a hierarchy
- **cpu**: cpu.rs and register.rs moved to module `cpu`
- **registers.rs**: changed `pack_registers` arguments to `Register`

## v0.3.0 (2024-12-24)

### Feat

- **cpu.rs/opcode.rs**: finished opcode rewrite
- **register.rs**: reworked register pairs and the pack/unpack fns
- Experimenting with individual opcodes for each addr mode

## v0.2.0 (2024-12-24)

### Feat

- fully operational (non-carry) u8 math opcodes

## v0.1.0 (2024-12-24)
