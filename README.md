### To run cargo asm commands with just.exe

CMD: (on windows Git bash is the easiest to use ... )
```
just asm stack
just asm heap
```

#### Deps: 
> https://github.com/gnzlbg/cargo-asm

> https://github.com/casey/just 

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