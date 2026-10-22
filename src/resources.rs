use macroquad::prelude::*;

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
    pub level_grass1: Texture2D
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
            level_grass1
        }
    }
}