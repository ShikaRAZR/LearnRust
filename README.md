# Learn Rust
My notes for learning what Rust is and how to use it


### Playlists/Links
- [Rust Lang Book](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- [Rust Lang Book YT Playlist](https://youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8&si=jgGR7tDL-xi_FO5L)
- [Rust Naming Convention](https://rust-lang.github.io/api-guidelines/naming.html)
- [Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
<br><br>

### Cheatsheet
> Compiling and Running are separate

#### rustc
        rustc main.rs
> ^ Compiling

        .\main.exe
> ^ Running on windows

#### cargo
        cargo new hello_cargo
        cd hello_cargo
> ^ Create Cargo project

        cargo new hello_world --bin
        cargo new hello_world --lib
        cargo new hello_cargo --vcs none
> ^ Create Cargo project, if we are making a binary program, library or no git repo

        cargo build
        .\target\debug\hello_cargo.exe
> ^ build and run hello_cargo, exe is in target folder

        cargo run
> ^ short hand for building cargo and running hello_cargo.exe

        cargo check
> ^ only checks if code compiles correctly

        git clone example.org/someproject
        cd someproject
        cargo build
> ^ check out an existing project

        cargo update
> ^ ignores Cargo.lock, only updates to a certain point, cargo version 0.8.5 would only update less than 0.9.0

        cargo doc --open
> ^ build documentation of all your dependencies and open in browser
<br><br>



### Info
- [hello_world](./hello_world) rustc - good for compiling simple programs 

- [hello_cargo](./hello_cargo) cargo - tool for real world Rust programs, adds dependencies
    - it creates src directory with main.rs, Cargo.toml and a git repository
    - toml -  (Tom’s Obvious, Minimal Language) format, cargo’s configuration format.
        - dependencies aka crates
    - top level directory outside src folder is for readme, license, config files, other unrelated coding files
    - convert rustc project to use cargo by using src folder and Cargo.toml file

- Cargo.toml

        [dependencies] 
        rand = "0.8.5"

> rand is a crate, "0.8.5" means any version at least 0.8.5 but below 0.9.0., it will also grab other dependencies that rand needs

- Cargo.lock, all depedencies with specified versions used, project will remain at 0.8.5 unless you explicitly upgrade, it is used to reproduce builds

- Rust has strong, static type system (must know types of all variables) and also type inference (compiler infers what type we use based on value and how we use it)
    - rust has 4 primary scalar types: integers, floating-point numbers, Booleans, characters
        - Integers
            - i - Signed (uses last bit to determine +,-), u - Unsigned, 
            - Signed: -(2^(n - 1)) to 2^(n - 1) - 1
            - Unsigned: 0 to 2^n - 1
            - i8 - 8-bit signed int [-128 to 127], inclusive
            - u8 - 8-bit unsigned [0 to 255], inclusive
            - integer overflow will panic in debug mode, but when released Rust doesnt include checks for overflow that cause panics, use built in methods to handle them
    - Compound Types: tuples/arrays, both have fixed length (vectors can change length)
- Rust is an expression-based language, function bodies made up of statements, optionally ending in an expression
    - Statements are instructions that perform action, dont return a value (Ex: let y = 6;)
    - Expressions evaluate to a resultant value, (Ex: a macro, new scope block with curly brackets)

### Project Order
1. hello_world
2. hello cargo
3. guessing_game
4. variables

<br><br><br><br><br><br>

<span>
# Stuff
### Stuff
- Stuff
> Stuff
<br><br>
</span>