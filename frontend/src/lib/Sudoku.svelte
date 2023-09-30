<script lang="ts">
    import {onMount} from 'svelte';
    import SudokuGrid from "$lib/SudokuGrid.svelte";

    let board = Array.from({length: 9}, () => Array(9).fill(0));
    let error = false;

    // Convert board to a single string
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
        const params = new URLSearchParams(window.location.search);
        const boardStr = params.get('board');
        if (boardStr && boardStr.length === 81) {
            board = stringToBoard(boardStr);
        } else {
            error = true
        }
    });

    // Function to update query string
    function updateQueryString() {
        const boardStr = boardToString(board);
        const newUrl = new URL(window.location.href);
        newUrl.searchParams.set('board', boardStr);
        history.replaceState(null, '', newUrl.toString());
    }
</script>

<!-- Error Banner -->
{#if error}
    <div class="bg-red-500 text-white p-4 rounded-lg shadow-md mb-4">
        <div class="flex items-center">
            <div class="flex-shrink-0">
                <!-- You can add an error icon here if you want -->
            </div>
            <div class="ml-3">
                <p class="text-sm font-medium">
                    Error: Board string is not valid.
                </p>
            </div>
        </div>
    </div>
{/if}

<!-- The Sudoku grid -->
{#if !error}
    <SudokuGrid board={board}/>
{/if}
