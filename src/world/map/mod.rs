use bevy::prelude::{info, Resource};
use crate::world::map_coordinates::MapPoint;

pub mod generator;

#[derive(Resource)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub spawn_point: MapPoint
}

impl Map {
    pub(crate) fn debug(&self) -> String{
        let mut visualisation = String::new();

        for x in 0..self.tiles.len() {
            let row = self.tiles.get(x).unwrap();
            for z in 0..row.len() {
                let char = match row.get(z).unwrap().tile_type {
                    TileType::None => ' ',
                    _ => '#'
                };
                visualisation.push(char);
            }
            visualisation += "\n";
        }

        visualisation
    }
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

    pub fn empty() -> Self {
        Self::new(TileType::None, WallType::None,WallType::None,WallType::None,WallType::None)
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
