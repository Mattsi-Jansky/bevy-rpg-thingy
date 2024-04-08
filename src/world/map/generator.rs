use crate::world::map::{Map, Tile, TileType, WallType};
use crate::world::map_coordinates::MapPoint;
use rand::distributions::{Distribution, Uniform};

const DUNGEON_SIZE_X: usize = 50;
const DUNGEON_SIZE_Z: usize = 50;
const MIN_ROOMS: usize = 6;
const MAX_ROOMS: usize = 14;

pub fn generate_map() -> Map {
    let room_shapes: Vec<RoomShape> = vec![
        RoomShape::new(2, 2),
        RoomShape::new(4, 2),
        RoomShape::new(4, 4),
        RoomShape::new(4, 6),
        RoomShape::new(6, 6),
        RoomShape::new(8, 6),
        RoomShape::new(8, 8),
        RoomShape::new(8, 10),
        RoomShape::new(10, 10),
        RoomShape::new(11, 10),
        RoomShape::new(11, 11),
        RoomShape::new(12, 12),
        RoomShape::new(13, 14),
        RoomShape::new(14, 14),
        RoomShape::new(15, 15),
        RoomShape::new(15, 10),
        RoomShape::new(7, 14),
        RoomShape::new(4, 8),
        RoomShape::new(6, 11),
    ];
    let mut tiles: Vec<Vec<Tile>> = vec![];

    for _x in 0..DUNGEON_SIZE_X {
        let mut row = vec![];
        for _z in 0..DUNGEON_SIZE_Z {
            row.push(Tile::empty())
        }
        tiles.push(row);
    }

    let mut rng = rand::thread_rng();
    let num_rooms = Uniform::new(MIN_ROOMS, MAX_ROOMS).sample(&mut rng);
    let room_shapes_distribution = Uniform::new(0, room_shapes.len());
    let mut rooms = vec![];
    let mut possible_spawn_points = vec![];
    for _ in 0..num_rooms {
        let room_shape = room_shapes
            .get(room_shapes_distribution.sample(&mut rng))
            .unwrap();
        let max_x = DUNGEON_SIZE_X - room_shape.width - 1;
        let max_z = DUNGEON_SIZE_Z - room_shape.height - 1;
        if max_x <= room_shape.width || max_z <= room_shape.height {
            continue;
        }
        let location_distribution_x = Uniform::new(0, max_x);
        let location_distribution_z = Uniform::new(0, max_z);
        let point = MapPoint::new(
            location_distribution_x.sample(&mut rng),
            location_distribution_z.sample(&mut rng),
        );
        possible_spawn_points.push(MapPoint::new(
            point.x + Uniform::new(0, room_shape.width).sample(&mut rng),
            point.z + Uniform::new(0, room_shape.height).sample(&mut rng),
        ));
        rooms.push(Room::new(point, room_shape.width, room_shape.height));
    }

    for room in rooms {
        room.apply(&mut tiles);
    }

    Map {
        tiles,
        spawn_point: possible_spawn_points.first().unwrap().clone(),
    }
}

struct RoomShape {
    width: usize,
    height: usize,
}

impl RoomShape {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

struct Room {
    pub north_west_point: MapPoint,
    pub width: usize,
    pub height: usize,
}

impl Room {
    pub fn new(north_west_point: MapPoint, width: usize, height: usize) -> Self {
        Self {
            north_west_point,
            width,
            height,
        }
    }

    pub fn apply(&self, tiles: &mut Vec<Vec<Tile>>) {
        for x in self.north_west_point.x..self.north_west_point.x + self.width {
            for z in self.north_west_point.z..self.north_west_point.z + self.height {
                let tile = tiles.get_mut(x).unwrap().get_mut(z).unwrap();
                tile.tile_type = TileType::Dirt;
                if x == self.north_west_point.x {
                    tile.wall_east = WallType::Regular;
                } else if x == self.north_west_point.x + self.width - 1 {
                    tile.wall_west = WallType::Regular ;
                }
                if z == self.north_west_point.z {
                    tile.wall_south = WallType::Regular;
                } else if z == self.north_west_point.z + self.height - 1 {
                    tile.wall_north = WallType::Regular;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn print_dungeon() {
        println!("{}", generate_map().debug());
    }
}
