use bevy::prelude::Resource;

pub mod generator;

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>
}

pub struct Tile {
    pub tile_type: TileType,
    pub wall_south: WallType,
    pub wall_west: WallType,
    pub wall_north: WallType,
    pub wall_east: WallType,
}

impl Tile {
    fn new(
        tile_type: TileType,
        wall_north: WallType,
        wall_east: WallType,
        wall_south: WallType,
        wall_west: WallType,
    ) -> Self {
        Self {
            tile_type,
            wall_south: wall_north,
            wall_west: wall_east,
            wall_north: wall_south,
            wall_east: wall_west,
        }
    }
}

pub enum TileType {
    None,
    Dirt,
    Tiled,
    Wood,
}

pub enum WallType {
    None,
    Regular,
}
