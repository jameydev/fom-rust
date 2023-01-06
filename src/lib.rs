#![allow(unused)]
pub mod game {
    pub mod utility {
        pub struct GameMessage {

        }
    }
    
    pub mod player {
        use super::items;
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
    
        pub struct Player<'a> {
            name: String,
            class: Class,
            race: Race,
            health: f32,
            mana: f32,
            inventory: Vec<&'a items::Item>
        }
    
        impl<'a> Player<'a> {
            pub fn new(name: &str, class: Class, race: Race) -> Self {
                Self { name: name.to_string(), class, race, health: 100.0, mana: 100.0, inventory: Vec::new() }
            }

            pub fn add_item(&mut self, item: &'a items::Item) {
                self.inventory.push(item);
            }

            pub fn get_inventory(&self) -> &Vec<&'a items::Item> {
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

    pub mod monsters {
        use super::items;
        pub struct Monster<'a> {
            name: String,
            health: f32,
            experience: usize,
            drops: Vec<&'a items::Item>
        }

        impl<'a> Monster<'a> {
            pub fn spawn(name: &str, health: f32, experience: usize, drops: Vec<&'a items::Item>) -> Monster<'a> {
                Monster { name: name.to_string(), health, experience, drops }
            }

            pub fn name(&self) -> &str {
                &self.name
            }

            pub fn drops(&self) -> &Vec<&'a items::Item> {
                &self.drops
            }

            pub fn take_damage(&mut self, dmg: f32) {
                self.health -= dmg;
            }

            pub fn kill(&self) {
                println!("Victory! You gained {}xp!", self.experience);
            }
        }
    }

    pub mod npc {

    }

    pub mod items {
        #[derive(Debug)]
        pub struct Item {
            name: String,
            qty: usize
        }

        impl Item {
            pub fn new(name: &str, qty: usize) -> Item {
                Item { name: name.to_string(), qty }
            }

            pub fn name(&self) -> &str {
                &self.name
            }

            pub fn qty(&self) -> usize {
                self.qty
            }
        }
    }
}

#[cfg(test)]
mod game_tests {
    use super::game::player::*;
    use super::game::items;

    fn player_setup() -> Player<'static> {
        Player::new("Gaius", Class::Rogue, Race::Human)
    }

    #[test]
    fn inventory_add_works() {
        let mut test_player = player_setup();
        let mut test_vec = Vec::new();
        
        let item1 = items::Item::new("Iron Sword", 1);
        let item2 = items::Item::new("Health Potion", 4);
        test_vec.push((item1.name(), item1.qty()));
        test_vec.push((item2.name(), item2.qty()));

        test_player.add_item(&item1);
        test_player.add_item(&item2);

        
        let mut vec2 = Vec::new();
        for item in test_player.get_inventory() {
            vec2.push((item.name(), item.qty()));
        }

        assert_eq!(test_vec, vec2);
    }

    #[test]
    fn monster_killer() {

    }
}