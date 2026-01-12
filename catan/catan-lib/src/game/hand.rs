use crate::object::{
    card::{DevCardType, DevelopmentCard, ResourceCard}, resource::{RESOURCE_NO, ResourceType}
};

/// Controls and handles all of the cards and unplayed structures.
pub struct Hand {
    rsrc_cards: [ResourceCard; RESOURCE_NO],
    development_cards: Vec<DevelopmentCard>,
}

impl Hand {
    /// It is very unlikely to hold very many development cards, so we do not use `with_capacity`.
    pub fn new() -> Self {
        Self {
            rsrc_cards: [
                ResourceCard::new(ResourceType::Wood, 0),
                ResourceCard::new(ResourceType::Brick, 0),
                ResourceCard::new(ResourceType::Wheat, 0),
                ResourceCard::new(ResourceType::Sheep, 0),
                ResourceCard::new(ResourceType::Ore, 0),
            ],

            development_cards: Vec::new(),
        }
    }

    pub fn get_resource(&self, resource: ResourceType) -> &ResourceCard {
        for card in self.rsrc_cards.iter() {
            if resource == card.get_resource() {
                return card;
            }
        }
        unreachable!()
    }

    pub fn add_resource_card(&mut self, resource: ResourceType, count: u32) {
        let card = self.get_mut_resource(resource);
        card.add(count)
    }

    pub fn sub_resource_card(&mut self, resource: ResourceType, count: u32) {
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

    fn get_mut_resource(&mut self, resource: ResourceType) -> &mut ResourceCard {
        for card in self.rsrc_cards.iter_mut() {
            if resource == card.get_resource() {
                return card;
            }
        }
        unreachable!()
    }
}
