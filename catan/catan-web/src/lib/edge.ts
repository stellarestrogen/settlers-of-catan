import type { WasmEdgePosition, WasmHexPosition } from "catan/catan_lib";
import type { GameData } from "./board_util";
import { BOARD_MARGIN_SIDE, EDGE_START_X, EDGE_START_Y, HEX_SIDE_LENGTH, HEX_WIDTH } from "./board_constants";

export function isEdgeEven(position: WasmEdgePosition) {
    return Math.abs((position.rights + position.downs) % 4) == 0 && Math.abs(position.downs % 2) == 0;
}

export function isEdgeOdd(position: WasmEdgePosition) {
    return Math.abs((position.rights + position.downs) % 4) == 2 && Math.abs(position.downs % 2) == 0;
}

export function isEdgePositive(position: WasmEdgePosition) {
    return Math.abs(position.downs % 2) == 1 && (Math.abs((position.rights + position.downs) % 4) == 0);
}

export function isEdgeInvalid(position: WasmEdgePosition) {
    return !isEdgeEven(position) && !isEdgeOdd(position) && !isEdgePositive(position);
}


function furthestRightEdge(width: number) {
    return width * 4;
}

function furthestDownEdge(height: number) {
    return height * 2 + 1;
}

export function nextEdge(position: WasmEdgePosition, width: number, height: number) {
    let nextPosition = Object.assign({}, position);

    if (isEdgeEven(position) || isEdgeOdd(position)) {
        nextPosition.rights += 2;
    } else if (isEdgePositive(position)) {
        console.log(`Edge ${nextPosition.rights} ${nextPosition.downs} is positive!`)
        nextPosition.rights += 4;
    }

    if (nextPosition.rights > furthestRightEdge(width)) {
        nextPosition.rights = -3;
        nextPosition.downs += 1;
    }

    while (isEdgeInvalid(nextPosition)) {
        console.log(`${nextPosition.rights} ${nextPosition.downs}`)
        nextPosition.rights += 1;
    }

    if (nextPosition.downs > furthestDownEdge(height)) {
        return null;
    }

    return nextPosition;
}

export function neighboringHexForEdge(position: WasmEdgePosition) {
    let hexes: WasmHexPosition[] = [];
    if (isEdgeEven(position)) {
        hexes = [
            {
                rights: Math.floor(position.rights / 4),
                downs: (position.downs - 2) / 2
            },
            {
                rights: Math.ceil(position.rights / 4),
                downs: position.downs / 2
            }
        ]
    } else if (isEdgeOdd(position)) {
        hexes = [
            {
                rights: Math.ceil(position.rights / 4),
                downs: (position.downs - 2) / 2
            },
            {
                rights: Math.floor(position.rights / 4),
                downs: position.downs / 2
            }
        ]
    } else if (isEdgePositive(position)) {
        hexes = [
            {
                rights: Math.ceil((position.rights + 1) / 4),
                downs: (position.downs - 1) / 2
            },
            {
                rights: Math.ceil((position.rights - 3) / 4),
                downs: (position.downs - 1) / 2
            }
        ]
    }
    return hexes;

}

export function edgeToCoordinates(position: WasmEdgePosition) {
    return [EDGE_START_X + position.rights * HEX_WIDTH / 4, EDGE_START_Y + position.downs * HEX_SIDE_LENGTH * 0.75]
}

export function edgePositions(data: GameData) {
    let positions: { positions: number[], nextPosition: WasmEdgePosition }[] = [];
    let currentPosition: WasmEdgePosition = { rights: -2, downs: 0 };
    let nextPosition;
    while ((nextPosition = nextEdge(currentPosition, data.width, data.height)) != null) {
        let isWater = true;
        for (let hex of neighboringHexForEdge(nextPosition)) {
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
        positions.push({ positions: edgeToCoordinates(nextPosition), nextPosition });
        currentPosition = nextPosition;
    }

    return positions;
}

export function rotateAngle(position: WasmEdgePosition) {
    if (isEdgeEven(position)) {
        return 60;
    } else if (isEdgeOdd(position)) {
        return -60;
    } else {
        return 0;
    }
}

export function rectangleCorners(position: WasmEdgePosition) {
    let center = edgeToCoordinates(position);
    let corners: number[] = [];
    if (isEdgeEven(position)) {
        corners = [

        ]
    } else if (isEdgeOdd(position)) {

    } else if (isEdgePositive(position)) {

    }
}