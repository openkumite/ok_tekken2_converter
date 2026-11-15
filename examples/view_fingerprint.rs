//! `cargo run --release --example view_fingerprint -- T3_PACK EXTRAS [LAYER...]`: the online fingerprint of the game's
//! view, the extras (the installed packs) over the packs' assets/ (LAYER...) over the Tekken 3 pack: what OpenKumite
//! computes.

use ok_converter_common::io::{Layered, StdFs, Vfs};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [t3, extras, layers @ ..] = &args[..] else {
        anyhow::bail!("usage: view_fingerprint T3_PACK EXTRAS [LAYER...]")
    };
    let top = StdFs(extras.into());
    let below: Vec<StdFs> = layers.iter().chain([t3]).map(|d| StdFs(d.into())).collect();
    let view = Layered { top: &top, below: below.iter().map(|l| l as &dyn Vfs).collect() };
    println!("{}", ok_tekken3_converter::convert_all::fingerprint(&view)?);
    Ok(())
}
