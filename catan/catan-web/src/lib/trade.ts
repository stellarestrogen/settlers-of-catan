import { WasmCornerPosition, type TradeType, type WasmTradePort } from "catan/catan_lib";
import { cornerToCoordinates, cornerToHex } from "./corner";
import { Rectangle, type GameData } from "./board_util";
import {
    CORNER_RADIUS,
    HEX_SIDE_LENGTH,
    HEX_WIDTH,
    TRADE_LINE_LENGTH,
    TRADE_LINE_WIDTH,
    TRADE_RADIUS,
} from "./board_constants";

export class DisplayTrade {
    constructor(
        public iconPosition: { x: number; y: number },
        public line1: Rectangle,
        public line2: Rectangle,
        public tradeType: TradeType,
    ) {}
}

function isVerticallyAligned(p1: WasmCornerPosition, p2: WasmCornerPosition) {
    return p1.rights == p2.rights;
}

export function tradeToCoordinates(position: WasmCornerPosition) {
    return cornerToCoordinates(position);
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

function calculateTradeLineAngle(icon: WasmCornerPosition, port: WasmCornerPosition) {
    if (icon.rights === port.rights) {
        return 0;
    } else if (
        (icon.rights - port.rights < 0 && icon.downs - port.downs < 0) ||
        (icon.rights - port.rights > 0 && icon.downs - port.downs > 0)
    ) {
        return -60;
    } else {
        return 60;
    }
}

function calculateRectangleCenter(corner: WasmCornerPosition, trade: WasmCornerPosition) {
    let realCorner = cornerToCoordinates(corner);
    let realTrade = cornerToCoordinates(trade);

    let angle = Math.acos(
        (realTrade[0] - realCorner[0]) /
            Math.sqrt((realTrade[0] - realCorner[0]) ** 2 + (realTrade[1] - realCorner[1]) ** 2),
    );

    if (trade.downs < corner.downs) angle = Math.PI * 2 - angle;

    let center = [
        0.5 * (realTrade[0] + realCorner[0]) +
            0.5 * (CORNER_RADIUS - TRADE_RADIUS) * Math.cos(angle),
        0.5 * (realTrade[1] + realCorner[1]) +
            0.5 * (CORNER_RADIUS - TRADE_RADIUS) * Math.sin(angle),
    ];

    return center;
}

export function displayTrades(ports: WasmTradePort[], offset: WasmCornerPosition, data: GameData) {
    let trades: DisplayTrade[] = [];
    ports = offsetTrades(ports, offset);
    for (let port of ports) {
        let tradePosition = findTradePosition(port.positions, data);
        let iconPosition = cornerToCoordinates(tradePosition);
        let center1 = calculateRectangleCenter(port.positions[0], tradePosition);

        let angle1 = calculateTradeLineAngle(tradePosition, port.positions[0]);

        let rect1 = new Rectangle(
            TRADE_LINE_WIDTH,
            TRADE_LINE_LENGTH,
            { x: center1[0], y: center1[1] },
            angle1,
        );

        let center2 = calculateRectangleCenter(port.positions[1], tradePosition);

        let angle2 = calculateTradeLineAngle(tradePosition, port.positions[1]);

        let rect2 = new Rectangle(
            TRADE_LINE_WIDTH,
            TRADE_LINE_LENGTH,
            {
                x: center2[0],
                y: center2[1],
            },
            angle2,
        );

        let trade = new DisplayTrade(
            { x: iconPosition[0], y: iconPosition[1] },
            rect1,
            rect2,
            port.trade,
        );
        trades.push(trade);
    }

    return trades;
}
