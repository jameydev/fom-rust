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
            Tiefling,
            Dragonkin
        }
    
        pub struct Player {
            name: String,
            class: Class,
            race: Race,
            health: f32,
            mana: f32,
            inventory: Vec<String>
        }
    
        impl Player {
            pub fn new(&self, name: &str, class: Class, race: Race) -> Self {
                Self { name: name.to_string(), class, race, health: 100.0, mana: 100.0, inventory: Vec::new() }
            }

            pub fn add_item(&mut self, item: &str) {
                self.inventory.push(item.to_string());
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
    #[test]
    fn inventory_add_works() {

    }
}