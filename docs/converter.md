# The converter and its tools

```
ok_tekken2_converter/
  src/            the converter; src/main.rs: t2_convert, a developer tool
  examples/       view_fingerprint.rs: the online fingerprint of the game's view (extras over packs over Tekken 3)
  viewer/         a browser viewer for the Tekken 2 disc's converted characters, stages and media
```

Ported from Python tools; it still writes their exact output (`pyjson`, `np`, `pil` reproduce Python's, numpy's
and Pillow's arithmetic). Nothing here imports the Tekken 3 converter: the install reads the Tekken 3 pack's files.

## What OpenKumite runs (`pack.rs`)

```sh
t2_convert pack "Tekken 2 (USA) (v1.1).cue" PACK                # the disc alone -> the Tekken 2 pack (~0.9 GB); any image format
t2_convert install PACK T3_PACK EXTRAS                          # the fighters, into the extras (~0.8 GB)
cargo run --release --example view_fingerprint -- T3_PACK EXTRAS PACK/assets
```

The game reads three layers: `EXTRAS` over the packs' `assets/` over `T3_PACK`; a file comes from the first that
has it. The install writes only into `EXTRAS`: the Tekken 3 pack stays as converted. With the Counter-Strike pack
too, its install goes first (`cs_levels install_pack`, into the same `EXTRAS`), then this one's: the order of
kumite_converter's `kumite_convert --t3 --cs --t2`, whose folder the three layers give byte for byte (checked: all
4728 files, fingerprint `912e1eb4...`, kumite_converter 0a20dae's).

## `t2_convert`, the rest

```sh
t2_convert "Tekken 2 (USA) (v1.1) (Track 1).bin" assets_t2         # the viewer's files: characters, stages, media
t2_convert transplant "Tekken 2 (USA) (v1.1) (Track 1).bin" assets  # the fighters into a Tekken 3 folder, in place
t2_convert export_characters|export_stages|export_media ARGS...    # one module
```

`transplant` reads the movies of `assets_t2/media/video/` beside `assets/` (the first command's media) and edits
the folder in place; then `t3_convert --fingerprint assets` (ok_tekken3_converter).

| Module (`src/`) | Output |
|---|---|
| `export_characters` | `characters/t2_block_NN.glb`: model, shadow hull, wings (Devil / Angel), every motion |
| `export_stages` | `stages/t2_stage_NN.glb`: floor patch, panorama drum, fills, lighting |
| `export_media` | `media/`: VAB sounds, XA music, STR movies, pictures, hit sparks |
| `transplant` | every Tekken 2 fighter as a playable fighter of OpenKumite (`model_9NN.glb`: costume block NN): glb on Tekken 3's skeleton, move set, roster entries, pictures |
| `pack` | the Tekken 2 pack and its install (what OpenKumite runs) |
| `transplant_moves` | their move sets: Tekken 2's attacks on a Tekken 3 donor's system |
| `ps1` | the PS1 formats these share (ISO 9660, TIM, glTF writer, SPU ADPCM, CD-XA, LZ) |
| `mips` | the MIPS interpreter that runs the game's own decoders |

The formats are in [reference/tekken2/engine.md](reference/tekken2/engine.md).

## Working on it

`sh check.sh commit` (the pre-commit hook): `cargo fmt`, clippy with warnings as errors, the tests; `sh check.sh push`
(the pre-push hook) adds `cargo deny`. A change to what the install writes changes the online fingerprint: check
it against the previous build's (the layers byte for byte, `view_fingerprint`). Floats as in ok_tekken3_converter
(its docs/converter.md, Working on it): only + - * /, `sqrt` in fixed orders, `ok_converter_common::np::fma`, the
`libm` crate's `sin`, `cos` and the rest.

## Viewer

```sh
serve 8767 viewer      # http://127.0.0.1:8767 (serve: ok_converter_common's)
```

It reads `assets_t2/` through links next to it (not in git): `models`, `stages`, `media` into `../assets_t2/`.
Create them with `ln -s`.
