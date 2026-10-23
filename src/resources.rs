use macroquad::audio::Sound;
use macroquad::audio::load_sound;
use macroquad::prelude::*;
use kira::StaticSoundData;

pub struct Resources {
    pub level_road1: Texture2D,
    pub level_road2: Texture2D,
    pub level_road3: Texture2D,
    pub level_road4: Texture2D,
    pub level_road5: Texture2D,
    pub level_road6: Texture2D,
    pub level_road7: Texture2D,
    pub level_road8: Texture2D,
    pub level_road9: Texture2D,
    pub level_road10: Texture2D,
    pub level_grass1: Texture2D,
    pub level_house1: Texture2D,
    pub level_house2: Texture2D,
    pub level_roof1: Texture2D,
    pub level_usps1: Texture2D,
    pub level_usps2: Texture2D,
    pub level_uspswin: Texture2D,
    pub level_hazard1: Texture2D,
    pub player_package: Texture2D,
    pub bgm: Sound,
}

impl Resources {
    pub async fn load() -> Resources {
        // i hate my life
        let level_road1 = load_texture("road1.png").await.expect("road1.png should exist");
        level_road1.set_filter(FilterMode::Nearest);
        let level_road2 = load_texture("road2.png").await.expect("road2.png should exist");
        level_road2.set_filter(FilterMode::Nearest);
        let level_road3 = load_texture("road3.png").await.expect("road3.png should exist");
        level_road3.set_filter(FilterMode::Nearest);
        let level_road4 = load_texture("road4.png").await.expect("road4.png should exist");
        level_road4.set_filter(FilterMode::Nearest);
        let level_road5 = load_texture("road5.png").await.expect("road5.png should exist");
        level_road5.set_filter(FilterMode::Nearest);
        let level_road6 = load_texture("road6.png").await.expect("road6.png should exist");
        level_road6.set_filter(FilterMode::Nearest);
        let level_road7 = load_texture("road7.png").await.expect("road7.png should exist");
        level_road7.set_filter(FilterMode::Nearest);
        let level_road8 = load_texture("road8.png").await.expect("road8.png should exist");
        level_road8.set_filter(FilterMode::Nearest);
        let level_road9 = load_texture("road9.png").await.expect("road9.png should exist");
        level_road9.set_filter(FilterMode::Nearest);
        let level_road10 = load_texture("road10.png").await.expect("road10.png should exist");
        level_road10.set_filter(FilterMode::Nearest);
        let level_grass1 = load_texture("grass1.png").await.expect("grass1.png should exist");
        level_grass1.set_filter(FilterMode::Nearest);
        let level_house1 = load_texture("house1.png").await.expect("house1.png should exist");
        level_house1.set_filter(FilterMode::Nearest);
        let level_house2 = load_texture("house2.png").await.expect("house2.png should exist");
        level_house2.set_filter(FilterMode::Nearest);
        let level_roof1 = load_texture("roof1.png").await.expect("roof1.png should exist");
        level_roof1.set_filter(FilterMode::Nearest);
        let level_usps1 = load_texture("usps1.png").await.expect("usps1.png should exist");
        level_usps1.set_filter(FilterMode::Nearest);
        let level_usps2 = load_texture("usps2.png").await.expect("usps2.png should exist");
        level_usps2.set_filter(FilterMode::Nearest);
        let level_uspswin = load_texture("uspswin.png").await.expect("uspswin.png should exist");
        level_uspswin.set_filter(FilterMode::Nearest);
        let level_hazard1 = load_texture("hazard1.png").await.expect("hazard1.png should exist");
        level_hazard1.set_filter(FilterMode::Nearest);
        let player_package = load_texture("package.png").await.expect("package.png should exist");
        player_package.set_filter(FilterMode::Nearest);
        let bgm = StaticSoundData::from_file("bgm.ogg")?;
        build_textures_atlas();

        Resources {
            level_road1,
            level_road2,
            level_road3,
            level_road4,
            level_road5,
            level_road6,
            level_road7,
            level_road8,
            level_road9,
            level_road10,
            level_grass1,
            level_house1,
            level_house2,
            level_roof1,
            level_usps1,
            level_usps2,
            level_uspswin,
            level_hazard1,
            player_package,
             bgm
        }
    }
}