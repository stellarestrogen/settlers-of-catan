<script lang="ts">
    import {
        HEX_WIDTH,
        HEX_HEIGHT,
        ROLL_NUMBER_RADIUS,
        FONT_SIZE,
        PROBABILITY_RADIUS,
        PROBABILITY_MARGIN,
    } from "./hex_constants";

    import * as util from "./board_util";

    let { tiles, height, width } = $props();

    let board_width = $derived(HEX_WIDTH * width + HEX_WIDTH);
    let board_height = $derived(HEX_HEIGHT * height);

    $inspect(board_width, board_height);

    function index(x: number, y: number) {
        return x + y * width;
    }

    function tileType(x: number, y: number) {
        return tiles[index(x, y)].tile_type;
    }

    function rollNumber(x: number, y: number) {
        return tiles[index(x, y)].roll_number;
    }

    function onTileClick(x: number, y: number) {
        console.log(`This hexagon's position is ${x}, ${y}`);
    }
</script>

<svg width={board_width} height={board_height}>
    {#each Array(height) as _, y}
        {#each Array(width) as _, x}
            {#if tileType(x, y) != "Water"}
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <polygon
                    points={util.calculateTilePosition(x, y)}
                    fill={util.getColor(tileType(x, y))}
                    stroke="black"
                    stroke-width="0.25%"
                    onclick={() => {
                        onTileClick(x, y);
                    }}
                />

                {#if tileType(x, y) != "Water" && tileType(x, y) != "Desert"}
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
                        fill={util.isRollNumberCommon(rollNumber(x, y))
                            ? "red"
                            : "black"}
                        font-style={util.isRollNumberCommon(rollNumber(x, y))
                            ? "italic"
                            : "normal"}
                        onclick={() => {
                            onTileClick(x, y);
                        }}
                        >{rollNumber(x, y)}
                    </text>
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    {#each Array(util.rollProbabilityCircles(rollNumber(x, y))) as _, i}
                        <circle
                            cx={util.probabilityCircleStartPosition(
                                x,
                                y,
                                rollNumber(x, y),
                            ).x +
                                PROBABILITY_MARGIN * i}
                            cy={util.calculateProbabilityCirclePosition(x, y).y}
                            r={PROBABILITY_RADIUS}
                            fill={util.isRollNumberCommon(rollNumber(x, y))
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
</svg>

<style>
</style>
