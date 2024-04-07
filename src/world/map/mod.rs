use lazy_static::lazy_static;

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

lazy_static! {
    pub static ref MAP: Vec<Vec<Tile>> = vec![
        vec![
            Tile::new(
                TileType::Dirt,
                WallType::Regular,
                WallType::None,
                WallType::None,
                WallType::Regular
            ),
            Tile::new(
                TileType::Dirt,
                WallType::None,
                WallType::None,
                WallType::None,
                WallType::Regular
            ),
            Tile::new(
                TileType::Dirt,
                WallType::None,
                WallType::None,
                WallType::None,
                WallType::Regular
            ),
            Tile::new(
                TileType::Dirt,
                WallType::None,
                WallType::None,
                WallType::None,
                WallType::Regular
            ),
            Tile::new(
                TileType::Dirt,
                WallType::None,
                WallType::None,
                WallType::Regular,
                WallType::Regular
            )
        ],
        vec![
            Tile::new(
                TileType::Wood,
                WallType::Regular,
                WallType::None,
                WallType::None,
                WallType::None
            ),
            Tile::new(
                TileType::Wood,
                WallType::None,
                WallType::None,
                WallType::None,
                WallType::None
            ),
            Tile::new(
                TileType::Wood,
                WallType::None,
                WallType::None,
                WallType::None,
                WallType::None
            ),
            Tile::new(
                TileType::Wood,
                WallType::None,
                WallType::None,
                WallType::None,
                WallType::None
            ),
            Tile::new(
                TileType::Wood,
                WallType::None,
                WallType::None,
                WallType::Regular,
                WallType::None
            ),
        ],
        vec![
            Tile::new(
                TileType::Tiled,
                WallType::Regular,
                WallType::Regular,
                WallType::None,
                WallType::None
            ),
            Tile::new(
                TileType::Tiled,
                WallType::None,
                WallType::Regular,
                WallType::None,
                WallType::None
            ),
            Tile::new(
                TileType::None,
                WallType::None,
                WallType::Regular,
                WallType::None,
                WallType::None
            ),
            Tile::new(
                TileType::Tiled,
                WallType::None,
                WallType::Regular,
                WallType::None,
                WallType::None
            ),
            Tile::new(
                TileType::Tiled,
                WallType::None,
                WallType::Regular,
                WallType::Regular,
                WallType::None
            )
        ]
    ];
}
