use crate::hex::{bounds::HexPerimeter, position::HexPosition, table::HexTable};

#[test]
fn test_hex_data() {
    let mut hex_bounds = HexPerimeter::new();
    hex_bounds.expand(HexPosition::ORIGIN + HexPosition::LEFT + HexPosition::UP_LEFT);
    hex_bounds.expand(HexPosition::ORIGIN + 3 * HexPosition::DOWN_RIGHT);
    let mut hex_table = HexTable::<u32>::new(hex_bounds);
    for (i, position) in hex_table.get_bounds().area().enumerate().collect::<Vec<_>>() {
        hex_table.set(position, i as u32).expect("Lol");
    }

    println!("hex_table: {:?}", hex_table)
}
