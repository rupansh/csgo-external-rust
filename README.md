# External CSGO Hack

[![License: GPL v3](https://img.shields.io/badge/license-GPL%20v3-blue?style=for-the-badge&logo=GNU)](https://www.gnu.org/licenses/gpl-3.0)
[![RUST](https://img.shields.io/badge/made%20with-RUST-red.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

**Undetected rn, will probably be detected in a day or two (duh)**

# Why?
I had learnt rust a while ago but I hadn't used it much. I wanted to try this project for revising rust, learning win32api and cheat development. Since there weren't many external cheats written in rust for csgo so I wrote one. This project is intended as a minimum base/PoC. Even though it is undetected right now, it will be detected when signature checks for it are added. You can probably improve the cheat by using lower level memory calls, proper visibility checks etc.


# Will this be updated?
probably not, unless i find something example worthy! It was intended to be a base/PoC. If you want your own cheat, write it yourself!

# offsets?
https://github.com/frk1/hazedumper/blob/master/csgo.cs \
- replace `public const Int32 ` with `"`
- replace ` =` with `" =>`
- replace `;` with `,`
- finally update `src/offsets.rs` !

# Should I try this in-game?
cheating4rarts. this was merely a fun experiment. Do not sabotage games! feel free use it in hvh