pub mod game {
    pub mod player {
        pub enum Class {
            Fighter,
            Rogue,
            Mage,
            Cleric
        }
    
        pub enum Race {
            Human,
            Dwarf,
            Elf,
            Ork,
            Halfling,
            Infernali,
            Dragonkin
        }
    
        pub struct Player {
            name: String,
            class: Class,
            race: Race,
            health: f32,
            mana: f32,
            inventory: Vec<(String, usize)>
        }
    
        impl Player {
            pub fn new(name: &str, class: Class, race: Race) -> Self {
                Self { name: name.to_string(), class, race, health: 100.0, mana: 100.0, inventory: Vec::new() }
            }

            pub fn add_item(&mut self, item: (String, usize)) {
                self.inventory.push(item);
            }

            pub fn get_inventory(&self) -> &Vec<(String, usize)> {
                &self.inventory
            }

            pub fn change_name(&mut self, name: &str) {
                self.name = name.to_string();
            }

            pub fn take_damage(&mut self, damage: f32) {
                self.health -= damage;
            }
        }

    }
}

#[cfg(test)]
mod game_tests {
    use std::vec;

    use super::game::player::*;

    fn player_setup() -> Player {
        Player::new("Gaius", Class::Rogue, Race::Human)
    }

    #[test]
    fn inventory_add_works() {
        let mut test_player = player_setup();
        test_player.add_item((String::from("Iron Sword"), 1));
        test_player.add_item((String::from("Health Potion"), 4));

        assert_eq!(vec![(String::from("Iron Sword"), 1), (String::from("Health Potion"), 4)], *test_player.get_inventory());
    }
}