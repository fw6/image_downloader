use anyhow::{anyhow, Result};
use clap::Parser;
use imagekit::avatar::Avatar;

#[derive(Parser, Debug, Clone)]
pub struct AvatarArgs {}

pub async fn gen_avatar(args: Avatar) -> Result<()> {
    let img = args.draw();

    img.save("avatar.png")
        .map_or(Err(anyhow!("save image failed")), |_| Ok(()))
}
