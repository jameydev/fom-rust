pub mod game {
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
        health: usize,
        mana: usize,
        inventory: Vec<String>
    }

    impl Player {

    }
}

#[cfg(test)]
mod game_tests {

}