//! `t2_convert BIN [OUTDIR] [characters,stages,media]` (convert_all: the viewer's files), `t2_convert pack BIN PACK
//! [--skip endings,music]` and `t2_convert install PACK T3_PACK EXTRAS` (what OpenKumite runs: pack.rs), or `t2_convert SCRIPT ARGS...` for
//! one of the scripts: export_characters, export_stages, export_media, transplant.

use ok_converter_common::io::{Ctx, Layered, StdFs, Sub, print_line};
use ok_tekken2_converter::*;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rest = args.get(1..).unwrap_or(&[]);
    let r = match args.first().map(String::as_str) {
        Some("export_characters") => export_characters::main(rest),
        Some("export_stages") => export_stages::main(rest),
        Some("export_media") => export_media::main(rest),
        Some("transplant") => transplant::main(rest),
        Some("pack") => pack_main(rest),
        Some("install") => install_main(rest),
        _ => convert_all::main(&args),
    };
    if let Err(e) = r {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

/// `pack BIN PACK [--skip endings,music]`: the Tekken 2 pack of the disc image BIN (its CD-audio tracks beside it),
/// without the parts named.
fn pack_main(args: &[String]) -> anyhow::Result<()> {
    let (args, skip) = match args {
        [a @ .., flag, parts] if flag == "--skip" => (a, parts.split(',').collect()),
        a => (a, Vec::new()),
    };
    let [src, out] = args else { anyhow::bail!("usage: t2_convert pack BIN PACK [--skip endings,music]") };
    if let Some(s) = skip.iter().find(|s| !pack::OPTIONAL.iter().any(|o| o.0 == **s)) {
        anyhow::bail!("--skip {s:?}: the parts a conversion can leave out are {:?}", pack::OPTIONAL.map(|o| o.0));
    }
    let (track1, cdda) = ok_tekken2_converter::ps1::image(src)?;
    let ctx = Ctx { out: &StdFs(out.into()), log: &print_line, jobs: 1 };
    pack::convert(&ctx, &track1, &cdda, &skip)?;
    Ok(())
}

/// `install PACK T3_PACK EXTRAS`: the pack installed beside a Tekken 3 pack, into the extras' folder (written over).
fn install_main(args: &[String]) -> anyhow::Result<()> {
    let [pack_dir, t3, extras] = args else { anyhow::bail!("usage: t2_convert install PACK T3_PACK EXTRAS") };
    let (top, pack_fs, base) = (StdFs(extras.into()), StdFs(pack_dir.into()), StdFs(t3.into()));
    let assets = Sub { inner: &pack_fs, prefix: pack::ASSETS.into() };
    let view = Layered { top: &top, below: vec![&assets, &base] };
    let ctx = Ctx { out: &view, log: &print_line, jobs: 1 };
    pack::install(&ctx, &pack_fs)
}
