use std::iter;

use lazy_static::lazy_static;
use rand::prelude::*;

use crate::protocol::{Character, PlayerRole, Team, Tile};

pub const SIZE: usize = 5;

lazy_static! {
    static ref WORDS: Vec<String> = include_str!("wordlist.txt")
        .lines()
        .map(|x| x.trim().to_string())
        .filter(|x| x.len() > 0)
        .collect();
}

pub struct Board {
    tiles: Vec<Tile>,
    starting_team: Team,
}

impl Board {
    /// Creates a new board.
    pub fn new() -> Board {
        let mut rng = thread_rng();
        let starting_team = if rng.gen() { Team::Red } else { Team::Blue };
        let (blue_agents, red_agents) = match starting_team {
            Team::Red => (8, 9),
            Team::Blue => (9, 8),
        };

        let mut characters = iter::repeat(Character::Bystander)
            .take(7)
            .chain(iter::repeat(Character::BlueAgent).take(blue_agents))
            .chain(iter::repeat(Character::RedAgent).take(red_agents))
            .chain(iter::once(Character::Assassin))
            .collect::<Vec<_>>();
        characters.shuffle(&mut rng);

        let tiles = WORDS
            .choose_multiple(&mut rng, SIZE * SIZE)
            .map(|word| Tile {
                codeword: word.to_string(),
                character: characters.pop().unwrap(),
                spotted: false,
            })
            .collect();

        Board {
            tiles,
            starting_team,
        }
    }

    /// Returns the starting team.
    pub fn starting_team(&self) -> Team {
        self.starting_team
    }

    /// Returns the tiles.
    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }

    /// Returns tiles with non spotted characters hidden.
    pub fn tiles_for_role(&self, role: PlayerRole) -> Vec<Tile> {
        self.tiles
            .iter()
            .map(|tile| {
                let mut tile = tile.clone();
                if !tile.spotted && role != PlayerRole::Spymaster {
                    tile.character = Character::Unknown;
                }
                tile
            })
            .collect()
    }
}
