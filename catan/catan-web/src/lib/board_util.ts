import type { WasmTileData } from "catan/catan_lib";
import { HEX_SIDE_LENGTH, HEX_ROW_HEIGHT, HEX_WIDTH, HEX_CENTER_X, HEX_CENTER_Y, PROBABILITY_MARGIN, HEX_HEIGHT, BOARD_MARGIN_TOP, CORNER_DELTA_HEIGHT } from "./board_constants";

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
        calculateXOffset(y) + HEX_WIDTH * x,
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

export function startingCorner() {
    return hexVertices(0, 0)[1];
}

export function nextCorner(position: number[], width: number, height: number) {
    let nextPosition = position;
    if ((position[1] / CORNER_DELTA_HEIGHT) % 4 == 3) {
        nextPosition[1] -= CORNER_DELTA_HEIGHT;
    }
    
}

export function cornerPositions(width: number, height: number, tiles: WasmTileData[]) {
    let positions: number[][] = [];
    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            if (tiles[x + y * width].tile_type == "Water") {
                
            }
            let vertices = hexVertices(x, y);
            for (let vertex of vertices) {
                vertex[0] = +vertex[0].toFixed(5);
                vertex[1] = +vertex[1].toFixed(5);
                if (positions.map(p => `${p[0]}-${p[1]}`).indexOf(`${vertex[0]}-${vertex[1]}`) === -1) {
                    positions.push(vertex);
                }
            }
        }
    }
    console.log(positions);
    return positions;
}

export function calculateRollNumberPosition(x: number, y: number) {
    let x_pos = calculateXOffset(y) + HEX_CENTER_X + HEX_WIDTH * x;
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
    x_pos -= PROBABILITY_MARGIN / 2 * (rollProbabilityCircles(roll) - 1);

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
            return "#000000";
    }
}