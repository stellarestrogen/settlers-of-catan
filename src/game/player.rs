use super::objects::*;
use super::board::{MAX_ROADS, MAX_SETTLEMENTS, MAX_CITIES};

#[derive(Clone)]
#[derive(Copy)]
pub enum PlayerColor {
    Red,
    Blue,
    White,
    Brown,
    Green,
    Orange,
}

pub struct Player {
    total_points: usize,
    resource_card_wood: ResourceCard,
    resource_card_brick: ResourceCard,
    resource_card_wheat: ResourceCard,
    resource_card_sheep: ResourceCard,
    resource_card_ore: ResourceCard,
    unplayed_dev_card: Vec<DevCardType>,
    soldiers_played: usize,
    /// list of indexes in the array of edges that hold this player's roads.
    roads: Vec<usize>,
    longest_road: usize,
    roads_played: usize,
    /// list of indexes in the array of corners that hold this player's settlements and cities.
    buildings: Vec<usize>,
    settlements_played: usize,
    cities_played: usize,
    unlocked_trade_wood: bool,
    unlocked_trade_brick: bool,
    unlocked_trade_wheat: bool,
    unlocked_trade_sheep: bool,
    unlocked_trade_ore: bool,
    unlocked_trade_any: bool,
    has_longest_road: bool,
    has_largest_army: bool,
}

impl Player {
    pub fn new() -> Self {
        Player {
            total_points: 0,
            resource_card_wood: ResourceCard::new(ResourceType::Wood, 0),
            resource_card_brick: ResourceCard::new(ResourceType::Brick, 0),
            resource_card_wheat: ResourceCard::new(ResourceType::Wheat, 0),
            resource_card_sheep: ResourceCard::new(ResourceType::Sheep, 0),
            resource_card_ore: ResourceCard::new(ResourceType::Ore, 0),
            unplayed_dev_card: Vec::<DevCardType>::new(),
            soldiers_played: 0,
            roads: Vec::<usize>::with_capacity(MAX_ROADS),
            longest_road: 0,
            roads_played: 0,
            buildings: Vec::<usize>::with_capacity(MAX_SETTLEMENTS + MAX_CITIES),
            settlements_played: 0,
            cities_played: 0,
            unlocked_trade_wood: false,
            unlocked_trade_brick: false,
            unlocked_trade_wheat: false,
            unlocked_trade_sheep: false,
            unlocked_trade_ore: false,
            unlocked_trade_any: false,
            has_longest_road: false,
            has_largest_army: false,
        }
    }

    pub fn add_resource(&mut self, resource: ResourceType, amt: i32) {
        match resource {
            ResourceType::Wood  => self.resource_card_wood.add(amt),
            ResourceType::Brick => self.resource_card_brick.add(amt),
            ResourceType::Wheat => self.resource_card_wheat.add(amt),
            ResourceType::Sheep => self.resource_card_sheep.add(amt),
            ResourceType::Ore   => self.resource_card_ore.add(amt),
        }
        
    }

    pub fn sub_resource(&mut self, resource: ResourceType, amt: i32) {
        match resource {
            ResourceType::Wood  => self.resource_card_wood.sub(amt),
            ResourceType::Brick => self.resource_card_brick.sub(amt),
            ResourceType::Wheat => self.resource_card_wheat.sub(amt),
            ResourceType::Sheep => self.resource_card_sheep.sub(amt),
            ResourceType::Ore   => self.resource_card_ore.sub(amt),
        }
    
    }

    pub fn get_resource_count(self, resource: ResourceType) -> i32 {
        match resource {
            ResourceType::Wood  => self.resource_card_wood.get_count(),
            ResourceType::Brick => self.resource_card_brick.get_count(),
            ResourceType::Wheat => self.resource_card_wheat.get_count(),
            ResourceType::Sheep => self.resource_card_sheep.get_count(),
            ResourceType::Ore   => self.resource_card_ore.get_count(),
        }
        // self.resource_cards[resource as usize].get_count()
    }

    pub fn unlock_trade(&mut self, trade: TradeType) {
        match trade {
            TradeType::Resource(resource) => {
                match resource {
                    ResourceType::Wood  => self.unlocked_trade_wood = true,
                    ResourceType::Brick => self.unlocked_trade_brick = true,
                    ResourceType::Wheat => self.unlocked_trade_wheat = true,
                    ResourceType::Sheep => self.unlocked_trade_sheep = true,
                    ResourceType::Ore   => self.unlocked_trade_ore = true,
                }
            }
            TradeType::Any => self.unlocked_trade_any = true,
        }
    }




}