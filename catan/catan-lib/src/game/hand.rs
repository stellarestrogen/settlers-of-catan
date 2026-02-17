use crate::object::{
    card::{DevelopmentCard, DevelopmentCardType, ResourceCard, ResourceMap},
    resource::ResourceType,
};

/// Controls and handles all of the cards and unplayed structures.
#[derive(Debug)]
pub struct Hand {
    resource_cards: ResourceMap,
    development_cards: Vec<DevelopmentCard>,
}

impl Hand {
    /// It is very unlikely to hold very many development cards, so we do not use `with_capacity`.
    pub fn new() -> Self {
        Self {
            resource_cards: ResourceMap::empty(),
            development_cards: Vec::new(),
        }
    }

    pub fn add_resource_card(&mut self, resource: ResourceType, count: u32) {
        self.get_mut_resource(resource).add(count)
    }

    pub fn sub_resource_card(&mut self, resource: ResourceType, count: u32) {
        self.get_mut_resource(resource).sub(count)
    }

    pub fn add_development_card(&mut self, card: DevelopmentCard) {
        self.development_cards.push(card)
    }

    /// Searches for the first card of the specified type that has not been played already.
    pub fn play_development_card(&mut self, r#type: DevelopmentCardType) {
        if let Some(card) = self
            .development_cards
            .iter_mut()
            .find(|c| c.get_type() == r#type && c.is_played() == false)
        {
            card.play()
        }
    }

    /// Counts the number of victory points from all played VP Development cards.
    pub fn count_victory_points(&self) -> usize {
        self.development_cards
            .iter()
            .filter(|c| c.is_played_and(|c| c.is_victory_point()))
            .count()
    }

    pub fn get_resources(&self) -> ResourceMap {
        self.resource_cards
    }

    pub fn get_mut_resources(&mut self) -> &mut ResourceMap {
        &mut self.resource_cards
    }

    pub fn get_resource(&self, resource: ResourceType) -> ResourceCard {
        self.resource_cards.get(resource)
    }

    pub fn get_mut_resource(&mut self, resource: ResourceType) -> &mut ResourceCard {
        self.resource_cards.get_mut(resource)
    }
}
