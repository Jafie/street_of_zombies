# Street Of Zombies - A game developped with Rust
![CI Tests](https://github.com/Jafie/street_of_zombies/actions/workflows/rust.yml/badge.svg)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/Jafie/street_of_zombies/blob/main/LICENSE)
[![Sprites: CC-0 license](https://img.shields.io/badge/License-CC--0-blue.svg)](https://creativecommons.org/licenses/by-sa/3.0/)

My first project and experience with [Rust](https://www.rust-lang.org/) and the [Bevy](https://github.com/bevyengine/bevy) engine. It is a Gun-And-Run game.

## How to play

[ Espace ] ==> Fire.

[ Arrows on keyboard ] ==> Movements

## Want to try ?

[You can try the game on this webpage!](https://jafie.github.io/street_of_zombies/)

## Installation and requirements

### OS Dependencies
- Windows: Be sure that VS2019 build tools are installed.

- Linux: [(Bevy engine dependencies)](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)

### Pre-requirements
"Street of zombies" needs a new target for "wasm support" and the "cargo-make" tools
```sh
rustup target add wasm32-unknown-unknown
cargo install cargo-make
```


## Build
Rust should be installed on your computer.
Then, a simple "cargo run" on the directoy root should work:

```sh
# Builds an run the game on local
cargo make run --profile native
```

You can also build a Web Assembly version of the game
```sh
# Builds the Web Assembly version
cargo make --profile build-web
```

You can test a Web Assembly version in local on 127.0.0.0:4000 with the following command
```sh
# Builds the Web Assembly version and host on 127.0.0.0:4000
cargo make serve --profile build-web
```

## Next steps - EXTRA

- [x] Weapon system.
- [x] Projectile system with interface.
- [x] Diagonal movements.
- [ ] Soundtrack.
- [x] Ennemy movements.
- [x] Ennemy spawn.
- [x] Ennemy AI (currently, simple movements)
- [x] Sprite usage.
- [x] Map limit.
- [ ] Bonus (Change weapons and Projectile types).
- [x] Scoreboard + Player health.

## Screenshots and Gameplay

![image](https://drive.google.com/uc?export=view&id=1CFA4GKzNX9vR14ToMPwrb1tVhlxmrKY-)

## Assets Credits

Sprites licenced under "Creative Commons Attribution-ShareAlike 3.0". Contributors and license in the ["Credits.txt"](https://github.com/Jafie/street_of_zombies/blob/main/Credits.txt) file:
