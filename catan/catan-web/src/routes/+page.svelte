<script lang="ts">
    import Board from "$lib/Board.svelte";
    import { WasmInterface, type WasmCustomEdition } from "catan";

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

    let game = $derived(WasmInterface.new_base(2));

    let length = $derived(game.get_length());
    let width = $derived(game.get_width());

    let tile_data = $derived(game.get_tile_data());
    // $inspect(tile_data);
</script>

<Board tiles={tile_data} {length} {width} />

<style>

</style>
