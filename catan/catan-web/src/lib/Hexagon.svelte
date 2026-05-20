<script lang="ts">
    let { tile_type, roll_number, x, y } = $props();
    let tile = $derived(`${tile_type}_tile.png`);
    let number = $derived(`${roll_number == null ? 0 : roll_number}_roll.png`);
    function getPosition() {
        console.log(`This hex's position is ${x}, ${y}\n`);
    }
</script>

<span class="tile" style:--x={x} style:--y={y} onclick={getPosition}>
<img src={tile} alt="Hexagon" class="resource"/>
<img src={number} alt="Number" class="number"/>
</span>

<style>
    .tile {
        position: relative;
        display: inline-block;
        margin: 0;
        padding: 0;
    }
    .resource {
        margin: 0;
    }
    .number {
        position: absolute;
        top: 20px;
        left: 0;
        scale: 0.35;
        margin: 0;
    }
</style>