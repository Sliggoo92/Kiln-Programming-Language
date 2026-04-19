# Kiln Programming Language — Specification & Roadmap (v0.0.2)

---

## Current Status

**Compiler pipeline: FUNCTIONAL**

The following are fully implemented and working:

- Workspace and crate structure
- Token system
- Lexer
- Full parser (all language constructs)
- AST definitions
- LLVM IR code generation via inkwell
- JIT execution engine
- `main then ... end` entry point
- Function definitions with paren-less parameter syntax
- Return type annotation via `return:type`
- Variable declarations (`let`, `const`)
- All arithmetic, assignment, and comparison operators
- Logical keyword operators (`and`, `or`, `not`)
- Control flow parsing (`if/else if/else`, `while`, `for`, `loop`)
- Module import parsing (`use console;`, `use console.print;`)
- Export keyword parsing
- Fixed and 2D array type annotations
- Struct definition parsing
- Comments (`//`)

**Known stubs (parsed but not yet codegen'd):**

- `codegen_if` — placeholder, returns error if used
- `codegen_while` — placeholder, returns error if used
- `codegen_loop` — placeholder, returns error if used
- `console.print` — parsed but not wired to output

**First working Kiln program:**

```
main then
    return 0;
end
```

Produces valid LLVM IR and executes via JIT.

---

## Next Goals (v0.0.3 targets)

### Priority 1 — `console.print`

Wire `console.print("text")` to LLVM's external `printf`. This is the single most important next step because without visible output, nothing else can be meaningfully tested.

Target syntax:

```
use console;

main then
    console.print("Hello from Kiln!\n");
    return 0;
end
```

Work required:
- Declare `printf` as an external LLVM function in codegen
- Intercept `console.print` calls and redirect to `printf`
- Handle string literal values as `i8*` pointers in codegen
- Test with basic string output

### Priority 2 — Control flow codegen

Implement the three stubbed codegen methods using LLVM basic blocks:

- `codegen_if` — conditional branch with merge block
- `codegen_while` — loop block with condition check and back-edge
- `codegen_loop` / `break` / `continue` — infinite loop with exit tracking

### Priority 3 — Variable type tracking

Currently `let` defaults all variables to `i64`. Each variable needs to carry its declared type through from declaration to load/store so that `float`, `bool`, and `string` variables work correctly.

---

## Philosophy

Kiln is a **portable systems language** designed to balance low-level power with modern developer usability.

Core principles:

- Portable First — programs run anywhere via a bytecode VM with optional native compilation.
- Statement-Oriented — programs are written as clear instructions to the machine.
- English-Readable Syntax — keywords favored over symbolic shorthand.
- Mutable by Default — ease of use and familiarity.
- Optional Immutability — safety when desired.
- Explicit Structure — predictable behavior over hidden magic.
- Minimal Runtime Baggage — avoids heavyweight scripting environments.
- Fast Compile + Fast Run workflow.
- Engineer / Hacker Feel — transparent and mechanical programming experience.

---

## Language Identity

**Name:** Kiln

Industrial and production-inspired naming. Kiln symbolizes creation, machinery, and systems engineering.

**File extension:** `.kiln`

---

## Execution Model

Kiln uses a **top-level execution model**. Programs execute statements in file order unless inside a function. The entry point is the `main` block — no parentheses, no return type declaration required.

```
main then
    console.print("Program started");
    return 0;
end
```

This allows Kiln to function as a scripting language, a systems language, or an embeddable language without changing syntax.

---

## Programming Model

Kiln is **statement-oriented**. Programs are structured as ordered instructions.

```
if x > 0 then
    console.print("positive");
end
```

---

## Blocks

All executable blocks follow a unified structure:

```
KEYWORD ... then
    statements
end
```

Applies to conditionals, loops, functions, and future constructs. Single-line form is allowed:

```
if health <= 0 then die(); end
```

---

## Entry Point

The program entry point is the `main` block. It takes no parameters and requires no return type annotation. `return 0;` exits with a status code.

```
main then
    return 0;
end
```

---

## Functions

Functions use the `func` keyword. Parameters are declared without parentheses — each parameter is `name: type` separated by spaces. Return type uses the `return` keyword followed by a colon and type.

```
func add a: int b: int return:int then
    return a + b;
end
```

No return type needed for void functions:

```
func greet name: string then
    console.print(name);
end
```

---

## Type System

- Strong static typing.
- Type inference supported where obvious.
- Dynamic behavior allowed through controlled mechanisms.

### Mutability

Mutable by default:

```
let x = 5;
```

Immutable:

```
const y = 10;
```

---

## Type Annotation Syntax

Colon-based annotation. No space before `:`, optional space after.

```
let health:int = 100;
let stamina: int = 50;
```

---

## Primitive Types

```
int
float
bool
string
byte
ptr
```

---

## Arrays

Fixed-size arrays only. Jagged (2D) arrays supported.

```
let scores: int[10];
let grid: int[10][10];
```

Dynamic arrays are not part of the current spec. They may be introduced via standard library modules in a later version.

---

## Memory Model

Garbage collection enabled by default. Optional manual memory management via modules:

```
use memory.manual;

let buffer: ptr = alloc(1024);
free(buffer);
```

Safe by default, powerful when required.

---

## Scope Model

### Module-Level Scope

Variables declared at the top of a file, outside any function, are module-level. All functions in the same file can access them freely.

```
let health: int = 100;

func damage amount: int then
    health -= amount;
end
```

### Block Scope

Variables declared inside a block are destroyed when that block ends.

```
if health > 0 then
    let message: string = "alive";
end

console.print(message);   // ERROR: out of scope
```

### No Implicit Closure Capture

Functions do not silently capture outer variables. Shared state lives at module level. Cross-file access requires explicit import.

---

## Module System

Each `.kiln` file is a module. Nothing crosses file boundaries unless exported and imported.

### Exporting

```
// player.kiln
export let health: int = 100;

export func damage amount: int then
    health -= amount;
end
```

### Importing

Full module import — access via qualified name:

```
use console;
console.print("hello");
```

Symbol import — access directly:

```
use console.print;
print("hello");
```

Cross-file usage:

```
// combat.kiln
use player;

func resolve_hit then
    player.damage(10);
    player.health -= 5;
end
```

---

## Comment Syntax

```
// single-line comment
let x = 5;   // inline comment
```

---

## Operators

### Arithmetic — Symbol

```
+   addition
-   subtraction
*   multiplication
/   division
%   modulo
```

### Assignment — Symbol

```
=   assign
+=  add and assign
-=  subtract and assign
*=  multiply and assign
/=  divide and assign
++  increment
--  decrement
```

### Comparison — Symbol

```
>   greater than
<   less than
>=  greater than or equal
<=  less than or equal
==  equal to
!=  not equal to
```

### Logical — Keyword

```
and   logical AND
or    logical OR
not   logical NOT
```

---

## Control Flow

### Conditionals

```
if health <= 0 then
    die();
else if health < 20 then
    console.print("critical");
else then
    console.print("ok");
end
```

### While Loop

```
while alive then
    update();
end
```

### For Loop

```
for i: int = 0; i < 10; i++ then
    console.print(i);
end
```

### Infinite Loop

```
loop then
    if done then
        break;
    end
end
```

### Loop Control

```
break;      // exit the loop
continue;   // skip to next iteration
```

---

## Compilation Targets

1. Native Binary via LLVM JIT (current)
2. Kiln Bytecode (planned)
3. Embedded VM execution (planned)

---

## Intended Use Cases

- Systems programming
- Cross-platform tools
- Game development
- Embedded scripting
- Experimental systems research

---

## Non-Goals

- Not purely functional.
- Not scripting-only.
- Not tied to a specific engine or vendor.

---

## Development Philosophy

Kiln evolves iteratively. Early versions remain usable. Stability grows over time. Real-world experimentation is encouraged.

---

## Roadmap

### v0.0.3 — Output & Control Flow
- `console.print` wired to output
- `codegen_if` implemented
- `codegen_while` implemented
- `codegen_loop` / `break` / `continue` implemented
- Variable type tracking through codegen

### v0.1.0 — Functional Language (First Executable Release)
- Multi-function programs work
- Basic standard library (`console`, `math`)
- CLI: `kiln run file.kiln`
- Basic error messages

### v0.2.0 — Bytecode & Virtual Machine
- Bytecode instruction set designed
- Stack-based VM
- AST to bytecode compiler

### v0.3.0 — Language Identity Phase
- Full module system working
- Structs
- REPL prototype
- Formatter prototype

### v0.5.0 — Ecosystem & Self-Hosting Preparation
- Standard library: math, strings, collections, file I/O
- Package manager prototype
- Begin rewriting tooling in Kiln

### v1.0.0 — Self-Hosting Language
- Kiln compiler written in Kiln
- Stable language specification
- Full standard library
- Package ecosystem operational

---

**Version 0.0.2 — Compiler Pipeline Functional**
