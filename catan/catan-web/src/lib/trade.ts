import { WasmCornerPosition, type WasmTradePort } from "catan/catan_lib";
import { cornerToCoordinates, cornerToHex } from "./corner";
import type { GameData } from "./board_util";

function isVerticallyAligned(p1: WasmCornerPosition, p2: WasmCornerPosition) {
    return p1.rights == p2.rights;
}

export function tradeToCoordinates(position: WasmCornerPosition) {
    return cornerToCoordinates(position);
}

export function findTradePosition(positions: WasmCornerPosition[], data: GameData) {
    let p1 = positions[0].clone();

    let [low, high] = p1.is_low() ? [positions[0], positions[1]] : [positions[1], positions[0]];

    let [pos1, pos2] = isVerticallyAligned(low, high)
        ? [
              new WasmCornerPosition(low.rights - 1, low.downs + 1),
              new WasmCornerPosition(high.rights + 1, high.downs - 1),
          ]
        : [
              new WasmCornerPosition(low.rights, low.downs - 2),
              new WasmCornerPosition(high.rights, high.downs + 2),
          ];

    let tradePos = data.tileTypeByPosition(cornerToHex(pos1)) == "Water" ? pos1 : pos2;

    return tradePos;
}

export function offsetTrades(ports: WasmTradePort[], offset: WasmCornerPosition) {
    for (let port of ports) {
        let p1 = new WasmCornerPosition(port.positions[0].rights, port.positions[0].downs);
        let p2 = new WasmCornerPosition(port.positions[1].rights, port.positions[1].downs);

        port.positions = [
            new WasmCornerPosition(p1.rights - offset.rights, p1.downs - offset.downs),
            new WasmCornerPosition(p2.rights - offset.rights, p2.downs - offset.downs),
        ];
    }

    return ports;
}
