import type { WasmCornerPosition, WasmHexPosition } from "catan/catan_lib";
import {
    BOARD_MARGIN_SIDE,
    HEX_CENTER_X,
    BOARD_MARGIN_TOP,
    HEX_CENTER_Y,
    CORNER_START_X,
    CORNER_START_Y,
} from "./board_constants";
import { GameData } from "./board_util";

function furthestRightCorner(width: number) {
    return width * 2;
}

function furthestDownCorner(height: number) {
    return height * 3;
}

function nextCorner(position: WasmCornerPosition, width: number, height: number) {
    let nextPosition = Object.assign({}, position);
    if (Math.abs(position.downs) % 3 == 0) {
        nextPosition.downs -= 1;
    } else if (Math.abs(position.downs) % 3 == 1 || Math.abs(position.downs) % 3 == 2) {
        nextPosition.downs += 1;
    }
    nextPosition.rights += 1;

    if (nextPosition.rights > furthestRightCorner(width)) {
        nextPosition.rights = -1;
        nextPosition.downs += Math.abs(nextPosition.downs) % 3 == 0 ? 2 : 4;
    }

    if (nextPosition.downs > furthestDownCorner(height)) {
        return null;
    }
    return nextPosition;
}

function neighboringHexForCorner(position: WasmCornerPosition) {
    let hexes: WasmHexPosition[] = [];
    if (Math.abs(position.downs) % 3 == 0) {
        hexes = [
            {
                rights: Math.ceil(position.rights / 2),
                downs: position.downs / 3,
            },
            {
                rights: Math.floor((position.rights - 1) / 2),
                downs: position.downs / 3,
            },
            {
                rights: Math.floor(position.rights / 2),
                downs: position.downs / 3 - 1,
            },
        ];
    } else {
        hexes = [
            {
                rights: Math.ceil(position.rights / 2),
                downs: (position.downs - 2) / 3,
            },
            {
                rights: Math.floor((position.rights - 1) / 2),
                downs: (position.downs - 2) / 3,
            },
            {
                rights: Math.floor(position.rights / 2),
                downs: (position.downs + 1) / 3,
            },
        ];
    }

    return hexes;
}

function cornerToCoordinates(position: WasmCornerPosition) {
    return [
        CORNER_START_X + position.rights * HEX_CENTER_X,
        CORNER_START_Y + (position.downs * HEX_CENTER_Y) / 2,
    ];
}

export function cornerPositions(data: GameData) {
    let positions: { positions: number[]; nextPosition: WasmCornerPosition }[] = [];
    let currentPosition: WasmCornerPosition = { rights: -1, downs: -1 };
    let nextPosition;
    while ((nextPosition = nextCorner(currentPosition, data.width, data.height)) != null) {
        let isWater = true;
        for (let hex of neighboringHexForCorner(nextPosition)) {
            if (data.tileTypeByPosition(hex) == "Water") {
                continue;
            } else {
                isWater = false;
                break;
            }
        }
        if (isWater) {
            currentPosition = nextPosition;
            continue;
        }
        positions.push({
            positions: cornerToCoordinates(nextPosition),
            nextPosition,
        });
        currentPosition = nextPosition;
    }

    return positions;
}
