# street_of_zombies
![CI Tests](https://github.com/Jafie/street_of_zombies/actions/workflows/rust.yml/badge.svg)
[![MIT license](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/Jafie/street_of_zombies/blob/main/LICENSE)
[![Sprites: CC-0 license](https://img.shields.io/badge/License-CC--0-blue.svg)](https://creativecommons.org/licenses/by-sa/3.0/)

First Rust project and experience with the [Bevy](https://github.com/bevyengine/bevy) engine. It is a simple Gun-And-Run game in development.

## Installation

### OS Dependencies
- Windows: Be sure that VS2019 build tools are installed.

- Linux: [(Bevy engine dependencies)](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)


### Build
Rust should be installed on your computer.
Then, a simple "cargo run" on the directoy root should work:

```sh
# Switch to the correct version (latest release, default is main development branch)
git checkout latest
# Builds an run the game on local
cargo make run
```

You can also build a Web Assembly version of the game
```sh
# Switch to the correct version (latest release, default is main development branch)
git checkout latest
# Builds the Web Assembly version
cargo make --profile release build-web
```

## How to play

[ Espace ] ==> Fire.

[ Arrows on keyboard ] ==> Movements

## Next steps - TODO

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
