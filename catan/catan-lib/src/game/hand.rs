use crate::object::{
    DevCardType, DevelopmentCard, ResourceCard, Structure, resource::{RESOURCE_NO, ResourceType}
};

#[derive(Clone, Copy)]
pub struct OwnedStructures {
    settlements: u32,
    cities: u32,
    roads: u32,
    boats: u32,
}

impl OwnedStructures {
    pub fn new(settlements: u32, cities: u32, roads: u32, boats: u32) -> Self {
        Self {
            settlements,
            cities,
            roads,
            boats,
        }
    }

    pub fn add_structure(&mut self, structure: Structure) {
        match structure {
            Structure::Settlement => self.settlements += 1,
            Structure::City => self.cities += 1,
            Structure::Road => self.roads += 1,
            Structure::Boat => self.boats += 1,
        };
    }

    pub fn remove_structure(&mut self, structure: Structure) -> Result<(), ()>{
        match structure {
            Structure::Settlement => self.settlements.checked_sub(1).ok_or(())?,
            Structure::City => self.cities.checked_sub(1).ok_or(())?,
            Structure::Boat => self.boats.checked_sub(1).ok_or(())?,
            Structure::Road => self.roads.checked_sub(1).ok_or(())?,
        };

        Ok(())
    }

    pub fn get_structure(&self, structure: Structure) -> u32 {
        match structure {
            Structure::Settlement => self.settlements,
            Structure::City => self.cities,
            Structure::Road => self.roads,
            Structure::Boat => self.boats,
        }
    }
}

/// Controls and handles all of the cards and unplayed structures.
pub struct Hand {
    rsrc_cards: [ResourceCard; RESOURCE_NO],
    development_cards: Vec<DevelopmentCard>,
    structures: OwnedStructures
}

impl Hand {
    /// It is very unlikely to hold very many development cards at once, so we do not use `with_capacity`.
    pub fn new(structures: OwnedStructures) -> Self {
        Self {
            rsrc_cards: [
                ResourceCard::new(ResourceType::Wood, 0),
                ResourceCard::new(ResourceType::Brick, 0),
                ResourceCard::new(ResourceType::Wheat, 0),
                ResourceCard::new(ResourceType::Sheep, 0),
                ResourceCard::new(ResourceType::Ore, 0),
            ],

            development_cards: Vec::new(),

            structures
        }
    }

    fn get_resource(&self, resource: ResourceType) -> &ResourceCard {
        for card in self.rsrc_cards.iter() {
            if resource == card.get_resource() {
                return card;
            }
        }
        unreachable!()
    }

    fn get_mut_resource(&mut self, resource: ResourceType) -> &mut ResourceCard {
        for card in self.rsrc_cards.iter_mut() {
            if resource == card.get_resource() {
                return card;
            }
        }
        unreachable!()
    }

    pub fn add_resource_card(&mut self, resource: ResourceType, count: i32) {
        let card = self.get_mut_resource(resource);
        card.add(count)
    }

    pub fn sub_resource_card(&mut self, resource: ResourceType, count: i32) {
        let card = self.get_mut_resource(resource);
        card.sub(count)
    }

    pub fn add_development_card(&mut self, card: DevelopmentCard) {
        self.development_cards.push(card)
    }

    /// Searches for the first card of the specified type that has not been played already.
    pub fn play_development_card(&mut self, r#type: DevCardType) {
        if let Some(card) = self
            .development_cards
            .iter_mut()
            .find(|c| c.get_type() == r#type && c.is_played() == false)
        {
            card.play()
        }
    }

    /// Counts the number of victory points from all played VP Development cards.
    pub fn count_victory_points(&self) -> u32 {
        let mut points = 0;
        for card in self.development_cards.iter() {
            if card.get_type() == DevCardType::VictoryPoint && card.is_played() {
                points += 1;
            }
        }

        points
    }

    pub fn add_structure(&mut self, structure: Structure) {
        self.structures.add_structure(structure)
    }

    pub fn remove_structure(&mut self, structure: Structure) -> Result<(), ()> {
        self.structures.remove_structure(structure)
    }

    pub fn count_structure(&self, structure: Structure) -> u32 {
        self.structures.get_structure(structure)
    }
}
