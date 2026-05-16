<script lang="ts">
    let { tile_type, roll_number } = $props();
    let src = $derived(`${tile_type}_tile.png`)
</script>

<img src={src} alt="Hexagon"/>