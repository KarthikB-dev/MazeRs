use ggez::{graphics::Image, Context, GameResult};

pub struct TankAssets {
    pub body_sprites: Vec<Image>,
    pub turret_sprite: Image,
}

impl TankAssets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut body_sprites = Vec::new();
        for i in 1..=6 {
            let path = format!("/tank/red/body/{:04}.png", i);
            body_sprites.push(Image::from_path(ctx, path)?);
        }

        let turret_sprite = Image::from_path(ctx, "/tank/red/turret/0001.png")?;

        Ok(Self {
            body_sprites,
            turret_sprite,
        })
    }
}

pub struct GameAssets {
    pub tank: TankAssets,
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            tank: TankAssets::new(ctx)?,
        })
    }
}
