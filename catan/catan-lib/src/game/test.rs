#[allow(unused_imports)]
use std::num::NonZeroUsize;

#[allow(unused_imports)]
use hexgrid::{
    edge::position::{EdgeOrientation, EdgePosition},
    hex::position::HexPosition,
};

#[allow(unused_imports)]
use crate::{
    game::{Game, edition},
    object::{
        card::ResourceMap,
        resource::ResourceType,
        structure::{
            OwnedStructures,
            transport::{Transport, TransportType},
        },
    },
};

#[test]
fn longest_road_test() {
    let edition = edition::CustomEdition::of_size(3, 5)
        .with_owned_structures(OwnedStructures::new(5, 4, 30, 0))
        .build();

    let mut game = Game::new(edition, NonZeroUsize::new(2).unwrap());

    let start = (HexPosition::ORIGIN + HexPosition::DOWN_LEFT) + EdgeOrientation::RIGHT;

    let roads: [EdgePosition; 21] = [
        start.into(),
        start.go_down_left().into(),
        start.go_down_left().go_down_left().into(),
        start.go_down_left().go_down_left().go_down_right().into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_down_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_down_right()
            .go_down_left()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .go_up_left()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .go_up_left()
            .go_up_left()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_right()
            .go_up_right()
            .go_up_left()
            .go_up_left()
            .go_up_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_down_right()
            .go_right()
            .into(),
        start
            .go_down_left()
            .go_down_left()
            .go_down_right()
            .go_right()
            .go_right()
            .go_right()
            .into(),
    ];

    let player1 = game.get_player(0).unwrap().token();
    game.find_player_mut(player1).add_resources(
        ResourceMap::empty()
            .with_resource(ResourceType::Brick, 100)
            .with_resource(ResourceType::Wood, 100),
    );

    for road in roads {
        game.play_transport(Transport::new(TransportType::Road, player1), road)
            .unwrap()
    }

    let longest_road = game.calculate_longest_road(player1);

    println!("The longest road was calculated to be {:}", longest_road);

    assert_eq!(longest_road, 15);
}
