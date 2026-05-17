<script lang="ts">
    import Hexagon from "$lib/Hexagon.svelte";
    import {WasmInterface, type WasmCustomEdition} from "catan";

    let x = $state(3);
    let y = $state(3);

    let edition: WasmCustomEdition = $derived({
        shortest: x,
        longest: x + y - 1,
        resource_distr: null,
        roll_numbers: null,
        trade_distr: null,
        trade_gaps: null,
        owned_structures: null,
    })

    let game = $derived(WasmInterface.new_custom(edition, 2));
    
    let length = $derived(game.get_length());
    let width = $derived(game.get_width());

    let tile_data = $derived(game.get_tile_data());
    // $inspect(tile_data);
</script>

<input bind:value={x} type="number" class="shortest">
<input bind:value={y} type="number" class="longest">

<div class="board">
    {#each Array(width) as _, y}
        <div class="row">

            {#each Array(length) as _, x}
                <Hexagon {...tile_data[x + y*length]} />
            {/each}
            <br/>
        </div>
    {/each}
</div>

<style>
    .longest {
        margin: 0px;
    }

    .shortest {
        margin: 0px;
    }

    .board {
        scale: 0.5;
        position: relative;
        top: 0px;
        left: 0px;
        background-color: rgb(0, 106, 255);
    }

    .row:nth-child(odd) {
        position: relative;
        left: 167px;
        margin: 0px;
        margin-top: -105px;
        margin-bottom: -105px;
    }
</style>