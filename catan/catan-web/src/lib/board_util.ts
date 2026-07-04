import type { WasmHexPosition, WasmTileData } from "catan/catan_lib";

import {
    HEX_SIDE_LENGTH,
    HEX_ROW_HEIGHT,
    HEX_WIDTH,
    HEX_CENTER_X,
    HEX_CENTER_Y,
    PROBABILITY_MARGIN,
    BOARD_MARGIN_TOP,
    BOARD_MARGIN_SIDE,
} from "./board_constants";

export class GameData {
    constructor(
        public tileData: WasmTileData[],
        public width: number,
        public height: number,
    ) {}
    isPositionInvalid(position: WasmHexPosition) {
        if (position.rights >= this.width || position.rights < 0) {
            return true;
        }

        if (position.downs >= this.height || position.downs < 0) {
            return true;
        }

        return false;
    }

    tileTypeByPosition(position: WasmHexPosition) {
        if (this.isPositionInvalid(position)) {
            return "Water";
        }

        return this.tileData[position.rights + position.downs * this.width]?.tile_type ?? "Water";
    }

    tileTypeByXY(x: number, y: number) {
        if (this.isPositionInvalid({ rights: x, downs: y })) {
            return "Water";
        }

        return this.tileTypeByPosition({ rights: x, downs: y });
    }

    rollNumberByPosition(position: WasmHexPosition) {
        return this.tileData[position.rights + position.downs * this.width].roll_number;
    }

    rollNumberByXY(x: number, y: number) {
        return this.rollNumberByPosition({ rights: x, downs: y });
    }
}

export function strokeWidth() {
    return HEX_SIDE_LENGTH * 0.03;
}

export function positionToSVG(position: number[]) {
    return `${position[0]},${position[1]}`;
}

export function isRollNumberCommon(roll: number) {
    return roll == 6 || roll == 8;
}

function calculateXOffset(y: number) {
    return y % 2 == 0 ? HEX_WIDTH / 2 : 0;
}

export function hexVertices(x: number, y: number) {
    let bl = [
        calculateXOffset(y) + BOARD_MARGIN_SIDE + HEX_WIDTH * x,
        BOARD_MARGIN_TOP + HEX_ROW_HEIGHT + HEX_ROW_HEIGHT * y,
    ];

    let tl = [bl[0], bl[1] - HEX_SIDE_LENGTH];
    let top = [tl[0] + HEX_WIDTH / 2, tl[1] - HEX_SIDE_LENGTH / 2];
    let tr = [top[0] + HEX_WIDTH / 2, top[1] + HEX_SIDE_LENGTH / 2];
    let br = [tr[0], tr[1] + HEX_SIDE_LENGTH];
    let bottom = [br[0] - HEX_WIDTH / 2, br[1] + HEX_SIDE_LENGTH / 2];

    return [bl, tl, top, tr, br, bottom];
}

export function calculateTilePosition(x: number, y: number) {
    let array = hexVertices(x, y);

    let positions = array.map(positionToSVG).join(" ");

    return positions;
}

export function calculateRollNumberPosition(x: number, y: number) {
    let x_pos = calculateXOffset(y) + BOARD_MARGIN_SIDE + HEX_CENTER_X + HEX_WIDTH * x;
    let y_pos = BOARD_MARGIN_TOP + HEX_CENTER_Y + HEX_ROW_HEIGHT * y;

    return { x: x_pos, y: y_pos };
}

export function rollProbabilityCircles(roll: number) {
    switch (roll) {
        case 2:
        case 12:
            return 1;
        case 3:
        case 11:
            return 2;
        case 4:
        case 10:
            return 3;
        case 5:
        case 9:
            return 4;
        case 6:
        case 8:
            return 5;
        default:
            return 0;
    }
}

export function calculateProbabilityCirclePosition(x: number, y: number) {
    let { x: x_pos, y: y_pos } = calculateRollNumberPosition(x, y);
    y_pos += HEX_SIDE_LENGTH / 6;

    return { x: x_pos, y: y_pos };
}

export function probabilityCircleStartPosition(x: number, y: number, roll: number) {
    let { x: x_pos, y: y_pos } = calculateProbabilityCirclePosition(x, y);
    x_pos -= (PROBABILITY_MARGIN / 2) * (rollProbabilityCircles(roll) - 1);

    return { x: x_pos, y: y_pos };
}

export function getColor(tile_type: String) {
    switch (tile_type) {
        case "Water":
            return "#0000ff";
        case "Desert":
            return "#ad9010";
        case "Wheat":
            return "#ffe675";
        case "Brick":
            return "#ba2f2f";
        case "Sheep":
            return "#a7ff24";
        case "Wood":
            return "#009612";
        case "Ore":
            return "#7a7a7a";
        default:
            return "#00000000";
    }
}
