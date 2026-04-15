# Kiln Language Specification (v0.0.1) 

## Philosophy

Kiln is a **portable systems language** designed to balance low-level power with modern developer usability. Core principles: - Portable First — programs run anywhere via a bytecode VM with optional native compilation. - Statement-Oriented — programs are written as clear instructions to the machine. - English-Readable Syntax — keywords favored over symbolic shorthand. - Mutable by Default — ease of use and familiarity. - Optional Immutability — safety when desired. - Explicit Structure — predictable behavior over hidden magic. - Minimal Runtime Baggage — avoids heavyweight scripting environments. - Fast Compile + Fast Run workflow. - Engineer / Hacker Feel — transparent and mechanical programming experience.

--- 
## Language Identity 
*Name:* Kiln Industrial and production-inspired naming. Kiln symbolizes creation, machinery, and systems engineering. 

--- 
## Execution Model 
Kiln uses a **top-level execution model**. Programs execute statements in file order unless inside a function. No mandatory main() or start() entry point exists. Example:
use console;

console.print("Program started");

This allows Kiln to function as: - a scripting language - a systems language - an embeddable language without changing syntax. --- ## Programming Model Kiln is **statement-oriented**. Programs are structured as ordered instructions. Example:

if x > 0 then
    console.print("positive");
end

--- 

## Blocks (Locked Design) All executable blocks follow a unified structure:

KEYWORD ... then
    statements
end

Applies to: - conditionals - loops - functions - future constructs Benefits: - predictable parsing - clear readability - consistent mental model --- ## Functions (Locked Design) Functions are defined using the func keyword.

func heal(amount: int) then
    health = health + amount;
end

Function syntax:
func name(parameters): return_type then
    statements;
end

Return type is optional if inferable. Example:

func add(a: int, b: int): int then
    return a + b;
end

--- 
## Type System - Strong static typing. - Type inference supported where obvious. - Dynamic behavior allowed through controlled mechanisms. 

### Mutability Mutable by default:
let x = 5;
Immutable variables:
const y = 10;

---
## Type Annotation Syntax (Locked) Colon-based annotation. Rules: - No space before : - Optional space after : Examples:
let health:int = 100;
let stamina: int = 50; 

---
## Primitive Types Initial primitive set:
int
float
bool
string
byte
ptr

---
## Memory Model - Garbage collection enabled by default. - Optional manual memory management via modules. Example:
use memory.manual;

let buffer: ptr = alloc(1024);
free(buffer);
Goal: Safe by default, powerful when required.

--- 
## Scope Model (Locked Design) ### Module-Level Scope Variables declared at the top of a file, outside any function, are **module-level**. All functions within the same file can access them freely — no special keywords required.
let health: int = 100;

func damage(amount: int) then
    health -= amount;
end

func heal(amount: int) then
    health += amount;
end

### Block Scope Variables declared inside a block (if, loop constructs, functions, etc.) are destroyed when that block ends. They do not leak outward.

if health > 0 then
    let message: string = "alive";
end

console.print(message);   // ERROR: message is out of scope

### No Implicit Closure Capture Functions do not silently capture variables from outer scopes. Shared state lives at module level in the file where it belongs. Functions in other files access it through explicit imports only.

--- 
## Module System (Locked Design) Each .soot file is a module. Nothing is shared across file boundaries unless explicitly exported and imported. ### Exporting Use the export keyword to make a symbol accessible to other modules:

// player.kiln
export let health: int = 100;

export func damage(amount: int) then
    health -= amount;
end

Symbols without export are private to that file. ### Importing Two valid forms: **Full module import** — imports the whole module, accessed via qualified name:

use console;

console.print("hello");

**Symbol import** — imports a single exported symbol directly into the current namespace:

use console.print;

print("hello");

Both forms are legal. The tradeoff is on the programmer: a symbol import gives you the short unqualified name but only for that one symbol. Importing the full module gives access to everything it exports via the module.symbol pattern. Example of cross-file module usage:

// combat.kiln
use player;

func resolve_hit() then
    player.damage(10);
    player.health -= 5;
end

---
## Comment Syntax (Locked) Single-line comments use //:

// this is a comment
let x = 5;   // inline comment

--- 
## Operators (Locked Design) Soot draws a clear line between symbolic and keyword operators based on readability.
### Arithmetic — Symbol
+   addition
-   subtraction
*   multiplication
/   division
%   modulo

### Assignment — Symbol
=   assign
+=  add and assign
-=  subtract and assign
*=  multiply and assign
/=  divide and assign
++  increment
--  decrement

### Comparison — Symbol
>   greater than
<   less than
>=  greater than or equal
<=  less than or equal
==  equal to
!=  not equal to

### Logical — Keyword
and   logical AND
or    logical OR
not   logical NOT

Examples:

if health > 0 and stamina < 100 then
    stamina += 10;
end

if not alive or health <= 0 then
    die();
end

Rationale: symbols are used where mathematical convention is universal and dense expressions are expected. Keywords are used for logical operators where English readability adds clarity without creating wall-of-text problems. 

---
## Control Flow (Locked Design)
### Conditionals

if condition then
    statements;
end

With else branches:

if health <= 0 then
    die();
else if health < 20 then
    console.print("critical");
else then
    console.print("ok");
end

Single-line form allowed:

if health <= 0 then die(); end

### While Loop Repeats while condition is true:

while alive then
    update();
end

### For Loop C-style three-part loop without parentheses:

for i: int = 0; i < 10; i++ then
    console.print(i);
end

### Infinite Loop Runs until explicitly broken:

loop then
    if done then
        break;
    end
end

### Loop Control
break;      // exit the loop immediately
continue;   // skip to the next iteration

---
## Syntax Philosophy - Keywords preferred over abbreviations where practical. - Semicolons allowed and encouraged. - Explicit block endings. - Indentation optional for compilation (readability only). Single-line blocks allowed:

if health <= 0 then die(); end

--- 
## Compilation Targets 1. Kiln Bytecode (default) 2. Native Binary (optional) 3. Embedded VM execution 

--- 
## Intended Use Cases - Systems programming - Cross-platform tools - Game development - Embedded scripting - Experimental systems research 

--- 
## Non-Goals - Not purely functional. - Not scripting-only. - Not tied to a specific engine or vendor. 

--- 
## Development Philosophy Kiln evolves iteratively: - Early versions remain usable. - Stability grows over time. - Real-world experimentation encouraged. 

--- 
## Status **Version 0.0.1 — Foundational Language Definition**
