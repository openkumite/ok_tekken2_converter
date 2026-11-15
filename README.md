# ok_tekken2_converter

Converts your own Tekken 2 disc (PlayStation, USA v1.1, SLUS-00213) into a pack of fighters OpenKumite plays beside
Tekken 3's (OpenKumite: an open-source 3D fighting game engine, a separate project): Kazuya, Jun, Angel and the rest
of Tekken 2's cast in the roster, each on Tekken 3's skeleton and system with its own Tekken 2 attacks, with their
home arenas, music and endings.

> **Unofficial.** ok_tekken2_converter is not affiliated with, endorsed or sponsored by Bandai Namco Entertainment.
> TEKKEN is a trademark of Bandai Namco Entertainment Inc.; PlayStation is a trademark of Sony Interactive
> Entertainment. This repository contains no game data: you need your own legally obtained copy, which the
> converter reads on your computer. Do not ask for, or share, disc images or converted files.

**Players do not run it.** OpenKumite links this crate and converts in the game: its Game data screen takes the
Tekken 2 disc image beside the Tekken 3 one. This repository is the library and the developer tools for working on it.

## The library

- `pack::convert(ctx, track1, cdda)`: the disc alone into a pack (any `Vfs`): the disc's tables and archive the
  install reads, and the fighters' music and endings as files the game reads as they are (`assets/`).
- `pack::install(ctx, pack)`: the fighters into the game's view: `ctx.out` is the extras (where everything goes)
  over the pack's `assets/` over the Tekken 3 pack (ok_converter_common's `Layered`). Each fighter plays a Tekken 3
  donor's system, its motions copied onto the fighter by joint name, so the install needs the Tekken 3 pack.

The disc image is the raw `.bin` (`Tekken 2 (USA) (v1.1) (Track 1).bin`); the CD-audio tracks are read beside it.

## Repository

| | |
|---|---|
| `src/` | the converter; `src/main.rs`: `t2_convert`, the developer tool |
| `viewer/` | a browser viewer for the Tekken 2 disc's converted characters, stages and media |
| `docs/converter.md` | the modules, the developer tool, the viewer, how to check a change |
| `docs/reference/` | the reference notes: the original game and its formats |

It depends on [ok_converter_common](https://github.com/openkumite/ok_converter_common); its tests and examples on
[ok_tekken3_converter](https://github.com/openkumite/ok_tekken3_converter) (the fingerprint). `sh check.sh` runs the checks.

## License

MIT ([LICENSE](LICENSE)). Third-party work: [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
