<script lang="ts">
    import {onMount} from 'svelte';
    import SudokuGrid from "$lib/SudokuGrid.svelte";

    let board = Array.from({length: 9}, () => Array(9).fill(0));
    const defaultBoard = "070004130000207006005013020001002000002190057003045802010378260367000580809001070"

    // Convert board to a single string is needed for api
    function boardToString(board: Array<Array<number>>) {
        return board.flat().join('');
    }

    // Convert a single string to board
    function stringToBoard(str: String) {
        let arr = Array.from(str).map(Number);
        return Array.from({length: 9}, (_, i) => arr.slice(i * 9, (i + 1) * 9));
    }

    // Initialize board from query string
    onMount(() => {
        //todo: replace with random board-generation?
        board = stringToBoard(defaultBoard)
    });

</script>


<!-- The Sudoku grid -->

<div class="flex justify-center items-center">
    <SudokuGrid board={board}/>
</div>

