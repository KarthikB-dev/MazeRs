use ggez::{Context, GameResult, graphics::Image};

pub struct EnvironmentAssets {
    pub ground_sprite:      Image,
    pub wall_sprite:        Image,
    pub button_sprite:      Image,
    pub local_flag_sprite:  Image,
    pub remote_flag_sprite: Image,
}

impl EnvironmentAssets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ground_sprite      = Image::from_path(ctx, "/env/grass.png")?;
        let wall_sprite        = Image::from_path(ctx, "/env/grass.png")?;
        let button_sprite      = Image::from_path(ctx, "/env/grass.png")?;
        let local_flag_sprite  = Image::from_path(ctx, "/env/grass.png")?;
        let remote_flag_sprite = Image::from_path(ctx, "/env/grass.png")?;

        Ok(Self {
            ground_sprite,
            wall_sprite,
            button_sprite,
            local_flag_sprite,
            remote_flag_sprite,
        })
    }
}

pub struct TankAssets {
    pub body_sprites: Vec<Image>,
    pub turret_sprite: Image,
}

impl TankAssets {
    pub fn new(ctx: &mut Context, color: &str) -> GameResult<Self> {
        let mut body_sprites = Vec::new();
        for i in 1..=6 {
            let path = format!("/tank/{}/body/{:04}.png", color, i);
            body_sprites.push(Image::from_path(ctx, path)?);
        }

        let turret_path = format!("/tank/{}/turret/0001.png", color);
        let turret_sprite = Image::from_path(ctx, turret_path)?;

        Ok(Self {
            body_sprites,
            turret_sprite,
        })
    }
}

pub struct GameAssets {
    pub local_tank:  TankAssets,
    pub remote_tank: TankAssets,
    pub environment: EnvironmentAssets
}

impl GameAssets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            local_tank:  TankAssets::new(ctx, "red")?,
            remote_tank: TankAssets::new(ctx, "blue")?,
            environment: EnvironmentAssets::new(ctx)?,
        })
    }
}
