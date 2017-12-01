mod resource_builder_set;
mod generator;

mod game;
pub use self::game::Game;

mod area;
pub use self::area::Area;

mod terrain;
pub use self::terrain::Terrain;

mod tile;
pub use self::tile::Tile;

mod actor;
pub use self::actor::Actor;

use std::collections::HashMap;
use std::rc::Rc;
use std::io::Error;

use resource::area::AreaBuilder;
use resource::resource_builder_set::ResourceBuilderSet;

pub struct ResourceSet {
    pub game: Game,
    pub areas: HashMap<String, Area>,
    tiles: HashMap<String, Rc<Tile>>,
    actors: HashMap<String, Rc<Actor>>,
}

pub trait ResourceBuilder where Self: Sized {
    fn owned_id(& self) -> String;

    fn new(data: &str) -> Result<Self, Error>;
}

impl ResourceSet {
    pub fn new(root_directory: &str) -> Result<ResourceSet, Error> {
        let builder_set = ResourceBuilderSet::new(root_directory)?;

        let tiles = create_rc_hashmap(builder_set.tiles);

        let mut areas: HashMap<String, Area> = HashMap::new();
        for (id, area_builder) in builder_set.area_builders {
            let area = Area::new(area_builder, &tiles);

            match area {
                Ok(a) => { areas.insert(id, a); }
                Err(e) => { eprintln!("{}", e); }
            }
        }

        Ok(ResourceSet {
            tiles: tiles,
            areas: areas,
            actors: create_rc_hashmap(builder_set.actors),
            game: builder_set.game,
        })
    }

    pub fn get_actor(&self, id: &str) -> Option<Rc<Actor>> {
        let actor = self.actors.get(id);

        match actor {
            None => None,
            Some(a) => Some(Rc::clone(a)),
        }
    }

    pub fn get_tile(&self, id: &str) -> Option<Rc<Tile>> {
        let tile = self.tiles.get(id);

        match tile {
            None => None,
            Some(t) => Some(Rc::clone(t)),
        }
    }
}

fn create_rc_hashmap<T>(data: HashMap<String, T>) -> HashMap<String, Rc<T>> {
    data.into_iter().map(|(id, entry)| (id, Rc::new(entry))).collect()
}
