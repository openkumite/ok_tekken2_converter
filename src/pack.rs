//! The Tekken 2 pack: what OpenKumite keeps of your Tekken 2 disc, made from the disc alone ([`convert`]), and its
//! install beside a Tekken 3 pack ([`install`]): the fighters brought onto Tekken 3's skeleton and system, written as
//! the files they add to or change in the Tekken 3 pack (the extras layer the game reads over it).
//!
//! A pack (any `Vfs`: a folder, or the browser's pack file):
//!
//! ```text
//! manifest.json        {"pack": "t2", "format": FORMAT, "source": {"file", "sha1_16mb", "missing"?}}
//! disc/SLUS_002.13     the disc's EXE: its tables (moves, motions, BGMs, sounds); the install reads them
//! disc/TEKKEN2.BNS     the disc's archive: models, motions, pictures, arenas, sounds
//! assets/              files the game reads as they are (a layer of its view, under the extras): the fighters'
//!                      stage music (audio/music/t2_NN.flac) and endings (video/1NN.avi, .flac)
//! ```
//!
//! The install needs the Tekken 3 pack (each fighter plays a Tekken 3 donor's system: its motions copied onto the
//! fighter by joint name, its reactions, throws and sounds). Into the view the game reads, the extras over the
//! pack's `assets/` over the Tekken 3 pack, it writes the rest of what `t2_convert transplant` writes into a
//! Tekken 3 conversion: together, that conversion byte for byte.

use crate::export_characters::{BNS_FILE, Disc, EXE_FILE};
use crate::export_media as em;
use crate::ps1::Iso;
use crate::pyjson;
use crate::transplant;
use anyhow::{Context, Result, bail};
use ok_converter_common::io::{Ctx, DiscFile, Sub, Vfs};
use serde_json::{Value, json};
use sha1::Digest;
use std::io::Read;

/// The pack's layout and contents; another one is converted again.
pub const FORMAT: i64 = 1;
/// What the install writes from a pack: when it changes, an installed pack is installed again (its pack stays as it is).
/// 2: the throw victims in install order, the walks' throws and Tekken 3 movement. 3: Tekken 2's own win poses, no
/// intros (the stance).
pub const INSTALL: i64 = 3;
/// What a pack's manifest names it.
pub const KIND: &str = "t2";
/// The disc's files a pack keeps (the pack's name, the disc's).
const DISC_FILES: [(&str, &str); 2] = [("disc/SLUS_002.13", EXE_FILE), ("disc/TEKKEN2.BNS", BNS_FILE)];
/// The pack's folder of files the game reads as they are.
pub const ASSETS: &str = "assets";
/// Where the movies are exported on the way (removed at the end).
const MEDIA: &str = ".tmp/media";

/// Converts the Tekken 2 disc (track 1 `src`, its CD-audio tracks `cdda`) into a pack in `ctx.out`, emptied first;
/// writes the manifest last and returns it.
pub fn convert(ctx: &Ctx, src: &dyn DiscFile, cdda: &[Option<Box<dyn DiscFile>>; 2]) -> Result<Value> {
    let out = ctx.out;
    out.remove_all("")?;
    let mut iso = Iso::open(src)?;
    ctx.log("[t2] the disc's tables and archive");
    for (to, from) in DISC_FILES {
        out.write(to, &iso.read(from)?)?;
    }
    let disc = Disc::open(src)?;
    let (music, movies) = transplant::disc_files(&disc)?;
    let assets = Sub { inner: out, prefix: ASSETS.into() };
    // (an image of the data alone, an ISO, has no XA sectors: no music, no endings; the install names them all the same)
    let media = src.whole_sectors();
    let music = if media { music } else { Vec::new() };
    let movies = if media { movies } else { Vec::new() };
    if !media {
        ctx.log(&format!(
            "[t2] {}: an image of the data alone (no Mode 2 Form 2 sectors): no music, no endings",
            src.name()
        ));
    }
    ctx.log("[t2] music");
    let xas_lba = iso.find("TEKKEN2/TEKKEN2.XAS")?.0;
    let streams = em::xa_streams(&disc)?;
    for (path, xa) in music {
        let stream = *streams.get(xa).with_context(|| format!("no XA stream {xa}"))?;
        em::export_stream(&mut iso, xas_lba, stream, &assets, &path)?;
    }
    ctx.log("[t2] endings");
    if media {
        em::run(ctx, src, cdda, MEDIA, &["video"])?;
    }
    let videos = Sub { inner: out, prefix: format!("{MEDIA}/video") };
    for (path, movie) in movies {
        if videos.exists(&movie) {
            assets.write(&path, &videos.read(&movie)?)?;
        }
    }
    out.remove_all(".tmp")?;
    let manifest = json!({"pack": KIND, "format": FORMAT, "source": source(src)?});
    out.write("manifest.json", pyjson::indent1(&manifest).as_bytes())?;
    out.checkpoint(&[])?;
    Ok(manifest)
}

/// The manifest of the pack in `pack`, checked: a Tekken 2 pack of this format.
pub fn manifest(pack: &dyn Vfs) -> Result<Value> {
    let m: Value = serde_json::from_slice(&pack.read("manifest.json")?).context("the pack's manifest.json")?;
    if m["pack"] != KIND {
        bail!("not a Tekken 2 pack");
    }
    if m["format"] != FORMAT {
        bail!("a Tekken 2 pack of another version: convert the Tekken 2 disc again");
    }
    Ok(m)
}

/// Installs the pack `pack` into `ctx.out`: the game's view, the extras (where the writes go) over the pack's
/// `assets/` ([`ASSETS`]) over the Tekken 3 pack (an `ok_converter_common::io::Layered`); the fingerprint is the
/// caller's.
pub fn install(ctx: &Ctx, pack: &dyn Vfs) -> Result<()> {
    let m = manifest(pack)?;
    let media = !m["source"]["missing"].as_array().is_some_and(|v| v.iter().any(|x| x == "movies"));
    let read = |to: &str| pack.read(to).with_context(|| format!("the Tekken 2 pack has no {to}"));
    let disc = Disc::from_files(read(DISC_FILES[0].0)?, read(DISC_FILES[1].0)?)?;
    // (the endings are in the view already: none are copied)
    let videos = Sub { inner: pack, prefix: ASSETS.into() };
    transplant::run_from(ctx, &disc, None, media, &videos, None)
}

/// The image's name and first 16 MB's SHA-1 (tells disc images apart), and what it does not hold ("missing":
/// "movies" and "XA music" from an image of the data alone; the game plays without them).
pub fn source(disc: &dyn DiscFile) -> Result<Value> {
    let mut head = Vec::new();
    disc.open()?.take(16 << 20).read_to_end(&mut head).with_context(|| format!("cannot read {}", disc.name()))?;
    let sha: String = sha1::Sha1::digest(&head).iter().map(|b| format!("{b:02x}")).collect();
    let mut v = json!({"file": disc.name(), "sha1_16mb": sha});
    if !disc.whole_sectors() {
        v["missing"] = json!(["movies", "XA music"]);
    }
    Ok(v)
}
