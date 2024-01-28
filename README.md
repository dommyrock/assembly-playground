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
Example output: 
fn stack()

![image](https://github.com/dommyrock/assembly-playground/assets/32032778/6cf53320-6ce9-48c4-b5e2-88e2f8b38e43)

fn heap()

![image](https://github.com/dommyrock/assembly-playground/assets/32032778/156bafa0-c08f-41fa-bdeb-0271614db281)

