use crate::object::{DevCardType, DevelopmentCard, ResourceCard, resource::{RESOURCE_NO, ResourceType}};


/// Controls and handles all of the cards.
pub struct Hand {
    rsrc_cards: [ResourceCard; RESOURCE_NO],
    development_cards: Vec<DevelopmentCard>

}


impl Hand {
    /// It is very unlikely to hold very many development cards at once, so we do not use `with_capacity`.
    pub fn new() -> Self {
        Self {
            rsrc_cards: [
                ResourceCard::new(ResourceType::Wood, 0),
                ResourceCard::new(ResourceType::Brick, 0),
                ResourceCard::new(ResourceType::Wheat, 0),
                ResourceCard::new(ResourceType::Sheep, 0),
                ResourceCard::new(ResourceType::Ore, 0)
            ],

            development_cards: Vec::new()
        }
    }

    fn get_resource(&self, resource: ResourceType) -> &ResourceCard {
        for card in self.rsrc_cards.iter() {
            if resource == card.get_resource() {
                return card
            }
        }
        unreachable!()
    }

    fn get_mut_resource(&mut self, resource: ResourceType) -> &mut ResourceCard {
        for card in self.rsrc_cards.iter_mut() {
            if resource == card.get_resource() {
                return card
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

    pub fn play_development_card(&mut self, r#type: DevCardType) {
        if let Some(card) = self.development_cards.iter_mut().find(|c| c.get_type() == r#type && c.is_played() == false) {
            card.play()
        }
    }

}