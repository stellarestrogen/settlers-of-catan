<script lang="ts">
    import Hexagon from "$lib/Hexagon.svelte";
    import {WasmInterface, type WasmCustomEdition} from "catan";

    let edition: WasmCustomEdition = {
        shortest: 1,
        longest: 2,
        resource_distr: null,
        roll_numbers: null,
        trade_distr: null,
        trade_gaps: null,
        owned_structures: null,
    }

    let game = WasmInterface.new_custom(edition, 2);
    
    let length = game.get_length();
    let width = game.get_width();

    let tile_data = $state(game.get_tile_data());
    // $inspect(tile_data);
</script>

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
    .board {

        position: relative;
        top: 200px;
        left: 100px;
    }

    .row:nth-child(odd) {
        position: relative;
        left: 167px;
        margin: 0px;
        margin-top: -105px;
        margin-bottom: -105px;
    }
</style>