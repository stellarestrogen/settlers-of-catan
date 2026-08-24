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
        EDGE_HEIGHT,
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
    import { displayTrades, findTradePosition, offsetTrades, tradeToCoordinates } from "./trade";
    import { cornerToCoordinates } from "./corner";

    let { tiles, trade_ports, height, width, game } = $props();

    let data = $derived(new util.GameData(tiles, trade_ports, width, height));

    let board_width = $derived(HEX_WIDTH * width + BOARD_MARGIN_SIDE * 2 + HEX_SIDE_LENGTH);
    let board_height = $derived(HEX_ROW_HEIGHT * (height - 1) + HEX_HEIGHT + BOARD_MARGIN_TOP * 2);

    function onTileClick(x: number, y: number) {
        let pos = new WasmHexPosition(x, y);
        game.take_hex_position(pos);
    }

    function onCornerClick(position: WasmCornerPosition) {
        // let a = new WasmCornerPosition(position.rights, position.downs);
        game.take_corner_position(position);
    }

    function onEdgeClick(position: WasmEdgePosition) {
        game.take_edge_position(position);
    }
</script>

<svg width={board_width} height={board_height}>
    <style>
        .corner {
            fill: burlywood;
            stroke: black;
            stroke-width: 1px;
        }

        .corner:hover {
            stroke: black;
            stroke-width: 2px;
        }
        .edge {
            fill: burlywood;
            stroke: black;
            stroke-width: 1px;
        }

        .edge:hover {
            stroke: black;
            stroke-width: 2px;
        }

        .trade {
            fill: white;
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
                    stroke="none"
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
            x={positions.rectangle.calculateTopLeft().x}
            y={positions.rectangle.calculateTopLeft().y}
            width={EDGE_WIDTH}
            height={EDGE_HEIGHT}
            transform="rotate({positions.rectangle.angle}, {positions.rectangle.center
                .x}, {positions.rectangle.center.y})"
            class="edge"
            data-position={positions.position}
            onclick={() => {
                onEdgeClick(positions.position);
            }}
        />
    {/each}

    {#each displayTrades(trade_ports, game.corner_offset(), data) as trade}
        <circle
            cx={trade.iconPosition.x}
            cy={trade.iconPosition.y}
            r={TRADE_RADIUS}
            class="trade"
        />

        <rect
            x={trade.line1.calculateTopLeft().x}
            y={trade.line1.calculateTopLeft().y}
            width={trade.line1.width}
            height={trade.line1.height}
            transform="rotate({trade.line1.angle}, {trade.line1.center.x}, {trade.line1.center.y})"
            fill="white"
        />

        <rect
            x={trade.line2.calculateTopLeft().x}
            y={trade.line2.calculateTopLeft().y}
            width={trade.line2.width}
            height={trade.line2.height}
            transform="rotate({trade.line2.angle}, {trade.line2.center.x}, {trade.line2.center.y})"
            fill="white"
        />
    {/each}
</svg>

<style>
</style>
