<script lang="ts">
    import {
        HEX_WIDTH,
        HEX_HEIGHT,
        ROLL_NUMBER_RADIUS,
        FONT_SIZE,
        PROBABILITY_RADIUS,
        PROBABILITY_MARGIN,
        CORNER_RADIUS,
        HEX_ROW_HEIGHT,
        BOARD_MARGIN_TOP,
        BOARD_MARGIN_SIDE,
        HEX_SIDE_LENGTH,
        EDGE_WIDTH,
        EDGE_LENGTH,
        TRADE_RADIUS,
    } from "./board_constants";

    import * as util from "./board_util";
    import * as corner from "./corner";
    import * as edge from "./edge";

    import {
        WasmCornerPosition,
        WasmHexPosition,
        type WasmEdgePosition,
        type WasmTradePort,
    } from "catan/catan_lib";
    import { findTradePosition, offsetTrades, tradeToCoordinates } from "./trade";

    let { tiles, trade_ports, height, width, game } = $props();

    let data = $derived(new util.GameData(tiles, trade_ports, width, height));

    let board_width = $derived(HEX_WIDTH * width + BOARD_MARGIN_SIDE * 2 + HEX_SIDE_LENGTH);
    let board_height = $derived(HEX_ROW_HEIGHT * (height - 1) + HEX_HEIGHT + BOARD_MARGIN_TOP * 2);

    function onTileClick(x: number, y: number) {
        let pos = new WasmHexPosition(x, y);
        console.log(`This hexagon's position is `, pos);
        game.take_hex_position(pos);
    }

    function onCornerClick(position: WasmCornerPosition) {
        // let a = new WasmCornerPosition(position.rights, position.downs);
        console.log(`This corner's position is `, position);
        game.query_trade(position);
    }

    function onEdgeClick(position: WasmEdgePosition) {
        console.log(`This edge's position is `, position);
    }
</script>

<svg width={board_width} height={board_height}>
    <style>
        .corner {
            fill: black;
        }

        .corner:hover {
            fill: red;
            stroke: black;
            stroke-width: 2px;
        }
        .edge {
            fill: black;
        }

        .edge:hover {
            fill: lime;
            stroke: black;
            stroke-width: 2px;
        }
    </style>
    {#each Array(height) as _, y}
        {#each Array(width) as _, x}
            {#if data.tileTypeByXY(x, y) != "Water"}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <polygon
                    points={util.calculateTilePosition(x, y)}
                    fill={util.getColor(data.tileTypeByXY(x, y))}
                    stroke="black"
                    stroke-width={util.strokeWidth()}
                    onclick={() => {
                        onTileClick(x, y);
                    }}
                />

                {#if data.tileTypeByXY(x, y) != "Desert"}
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <circle
                        cx={util.calculateRollNumberPosition(x, y).x}
                        cy={util.calculateRollNumberPosition(x, y).y}
                        r={ROLL_NUMBER_RADIUS}
                        fill="white"
                        stroke="black"
                        onclick={() => {
                            onTileClick(x, y);
                        }}
                    />
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <text
                        font-size={FONT_SIZE}
                        x={util.calculateRollNumberPosition(x, y).x}
                        y={util.calculateRollNumberPosition(x, y).y}
                        text-anchor="middle"
                        dominant-baseline="middle"
                        fill={util.isRollNumberCommon(data.rollNumberByXY(x, y)!) ? "red" : "black"}
                        font-style={util.isRollNumberCommon(data.rollNumberByXY(x, y)!)
                            ? "italic"
                            : "normal"}
                        onclick={() => {
                            onTileClick(x, y);
                        }}
                        >{data.rollNumberByXY(x, y)!}
                    </text>
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    {#each Array(util.rollProbabilityCircles(data.rollNumberByXY(x, y)!)) as _, i}
                        <circle
                            cx={util.probabilityCircleStartPosition(
                                x,
                                y,
                                data.rollNumberByXY(x, y)!,
                            ).x +
                                PROBABILITY_MARGIN * i}
                            cy={util.calculateProbabilityCirclePosition(x, y).y}
                            r={PROBABILITY_RADIUS}
                            fill={util.isRollNumberCommon(data.rollNumberByXY(x, y)!)
                                ? "red"
                                : "black"}
                            onclick={() => {
                                onTileClick(x, y);
                            }}
                        />
                    {/each}
                {/if}
            {/if}
        {/each}
    {/each}

    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    {#each corner.cornerPositions(data) as positions}
        <circle
            cx={positions.positions[0]}
            cy={positions.positions[1]}
            r={CORNER_RADIUS}
            class="corner"
            data-position={positions.nextPosition}
            onclick={() => {
                onCornerClick(positions.nextPosition);
            }}
        />
    {/each}

    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    {#each edge.edgePositions(data) as positions}
        <rect
            x={positions.positions[0] - EDGE_WIDTH / 2}
            y={positions.positions[1] - EDGE_LENGTH / 2}
            width={EDGE_WIDTH}
            height={EDGE_LENGTH}
            transform="rotate({edge.rotateAngle(positions.nextPosition)}, {positions
                .positions[0]}, {positions.positions[1]})"
            class="edge"
            data-position={positions.nextPosition}
            onclick={() => {
                onEdgeClick(positions.nextPosition);
            }}
        />
    {/each}

    {#each offsetTrades(trade_ports, game.corner_offset()) as port}
        <circle
            cx={tradeToCoordinates(findTradePosition(port.positions, data))[0]}
            cy={tradeToCoordinates(findTradePosition(port.positions, data))[1]}
            r={TRADE_RADIUS}
        />
    {/each}
</svg>

<style>
</style>
