<script lang="ts">
    let { tile_type, roll_number } = $props();
    let tile = $derived(`${tile_type}_tile.png`);
    let number = $derived(`${roll_number == null ? 0 : roll_number}_roll.png`);
</script>

<span class="tile">
<img src={tile} alt="Hexagon" class="resource"/>
<img src={number} alt="Number" class="number"/>
</span>

<style>
    .tile {
        position: relative;
        display: inline-block;
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