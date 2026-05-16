<script lang="ts">
    import Hexagon from "$lib/Hexagon.svelte";
    import {WasmInterface} from "catan";

    let game = WasmInterface.new_base(2);
    
    let length = game.get_length();
    let width = game.get_width();

    let tile_data = $state(game.get_tile_data());
    $inspect(tile_data);
</script>

{#each Array(width) as _, y}
    {#each Array(length) as _, x}
        <Hexagon {...tile_data[x + y*length]} />
    {/each}
    <br />
{/each}