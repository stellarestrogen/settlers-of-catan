<script lang="ts">
    import Board from "$lib/Board.svelte";
    import { type WasmHexPosition, WasmInterface, type WasmCustomEdition } from "catan";

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
    });

    let game = $derived(WasmInterface.new_custom(edition, 2));

    let height = $derived(game.get_height());
    let width = $derived(game.get_width());

    $inspect(height, width)

    let tile_data = $derived(game.get_tile_data());
    // $inspect(tile_data);
</script>

<input bind:value={x} type="number" class="shortest" min="1" />
<input bind:value={y} type="number" class="longest" min="2" />

<div class="board">
    <Board tiles={tile_data} {height} {width} {game}/>
</div>

<style>
    .board {
        position: relative;
        top: 100px;
    }
</style>
