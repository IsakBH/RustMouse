# PÅ LINUX FUNKER DET BARE PÅ XORG, INGEN ANELSE OM DET FUNKER PÅ WINDOWS OG MACOS

## Hvordan laste ned og builde
1. Clone repo-et med git
- `git clone https://github.com/isakbh/rustmouse`
2. Last ned og installer dependencies
  - [rustup](https://rust-lang.org)
  - [xdotool](https://github.com/jordansissel/xdotool)
3. Gå inn i rustmouse mappen du nettopp clonet
  - `cd rustmouse`
4. Kjør programmet med [cargo]([url](https://doc.rust-lang.org/cargo/))
- `cargo run`
- For å bare builde programmet, kjør `cargo build --release`

## To-Do
- [x] Beveging av musepeker
- [ ] Trykking (left click og right click)
- [x] Andre hotkeys for mer eller mindre presisjon
