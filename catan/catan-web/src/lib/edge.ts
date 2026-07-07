import { WasmEdgePosition, type WasmHexPosition } from "catan/catan_lib";
import type { GameData } from "./board_util";
import { EDGE_START_X, EDGE_START_Y, HEX_SIDE_LENGTH, HEX_WIDTH } from "./board_constants";

function furthestRightEdge(width: number) {
    return width * 4;
}

function furthestDownEdge(height: number) {
    return height * 2 + 1;
}

function nextEdge(position: WasmEdgePosition, width: number, height: number) {
    let nextPosition = new WasmEdgePosition(position.rights, position.downs);

    if (position.is_even() || position.is_odd()) {
        nextPosition.rights += 2;
    } else if (position.is_positive()) {
        nextPosition.rights += 4;
    }

    if (nextPosition.rights > furthestRightEdge(width)) {
        nextPosition.rights = -3;
        nextPosition.downs += 1;
    }

    while (nextPosition.is_invalid()) {
        nextPosition.rights += 1;
    }

    if (nextPosition.downs > furthestDownEdge(height)) {
        return null;
    }

    return nextPosition;
}

function edgeToCoordinates(position: WasmEdgePosition) {
    return [
        EDGE_START_X + (position.rights * HEX_WIDTH) / 4,
        EDGE_START_Y + position.downs * HEX_SIDE_LENGTH * 0.75,
    ];
}

export function edgePositions(data: GameData) {
    let positions: { positions: number[]; nextPosition: WasmEdgePosition }[] = [];
    let currentPosition = new WasmEdgePosition(-2, 0);
    let nextPosition;
    while ((nextPosition = nextEdge(currentPosition, data.width, data.height)) != null) {
        let isWater = true;
        for (let hex of nextPosition.neighboring_hex()) {
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
    if (position.is_even()) {
        return 60;
    } else if (position.is_odd()) {
        return -60;
    } else {
        return 0;
    }
}
