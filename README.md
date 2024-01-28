### To run cargo asm commands with just.exe

#### Deps: 
> https://github.com/gnzlbg/cargo-asm
> https://github.com/casey/just (on windows Git bash is the easiest to use ... )

#### Also important to rename crate to "stack" to match justifle 
```
[package]
name = "stack"
version = "0.1.0"
edition = "2021"
```

Justfile 
```
asm METHOD:
    cargo asm stack::{{METHOD}}
```