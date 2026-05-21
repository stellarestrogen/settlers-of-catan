<script lang="ts">
    const HEX_SIDE_LENGTH = 75;
    const HEX_WIDTH = Math.sqrt(3) * HEX_SIDE_LENGTH;
    const HEX_HEIGHT = HEX_SIDE_LENGTH * 2;

    let { tiles, length, width } = $props();

    let board_width = $derived(HEX_WIDTH * width + HEX_WIDTH);
    let board_height = $derived(HEX_HEIGHT * length);

    function positionToSVG(position: number[]) {
        return `${position[0]},${position[1]}`;
    }

    function tileType(x: number, y: number) {
        return tiles[x + y * length].tile_type;
    }

    function rollNumber(x: number, y: number) {
        return tiles[x + y * length].roll_number;
    }

    function calculatePosition(x: number, y: number) {
        let x_offset = y % 2 == 0 ? HEX_WIDTH / 2 : 0;

        let start = [
            x_offset + HEX_WIDTH * x,
            HEX_SIDE_LENGTH +
                HEX_SIDE_LENGTH / 2 +
                (HEX_SIDE_LENGTH + HEX_SIDE_LENGTH / 2) * y,
        ];

        let tl = [start[0], start[1] - HEX_SIDE_LENGTH];
        let top = [tl[0] + HEX_WIDTH / 2, tl[1] - HEX_SIDE_LENGTH / 2];
        let tr = [top[0] + HEX_WIDTH / 2, top[1] + HEX_SIDE_LENGTH / 2];
        let br = [tr[0], tr[1] + HEX_SIDE_LENGTH];
        let bottom = [br[0] - HEX_WIDTH / 2, br[1] + HEX_SIDE_LENGTH / 2];

        let array = [start, tl, top, tr, br, bottom];

        let positions = array.map(positionToSVG).join(" ");

        return positions;
    }

    function getColor(tile_type: String) {
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
</script>

<svg width={board_width} height={board_height}>
    {#each Array(width) as _, y}
        {#each Array(length) as _, x}
            <polygon
                points={calculatePosition(x, y)}
                fill={getColor(tileType(x, y))}
            />
        {/each}
    {/each}
</svg>

<style>
</style>
