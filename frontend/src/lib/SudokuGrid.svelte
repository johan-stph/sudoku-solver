<script lang="ts">
    export let board: number[][];

    let canvas: HTMLCanvasElement;

    // Reactive statement to re-draw the canvas whenever the board changes
    $: {
        if (canvas) {  // Check if canvas is defined
            const dpr = window.devicePixelRatio || 1;
            const rect = canvas.getBoundingClientRect();

            // Multiply canvas dimensions by device pixel ratio
            canvas.width = rect.width * dpr;
            canvas.height = rect.height * dpr;

            const ctx = canvas.getContext('2d')!;

            // Scale the drawing context by the device pixel ratio
            ctx.scale(dpr, dpr);

            const cellSize = 50; // Size of each cell
            const boardSize = 9 * cellSize;
            const thickLine = 4;
            const thinLine = 1;

            // Clear the canvas before redrawing
            ctx.clearRect(0, 0, boardSize, boardSize);

            // Draw grid
            for (let i = 0; i <= 9; i++) {
                ctx.lineWidth = i % 3 === 0 ? thickLine : thinLine;

                // Vertical lines
                ctx.beginPath();
                ctx.moveTo(i * cellSize, 0);
                ctx.lineTo(i * cellSize, boardSize);
                ctx.stroke();

                // Horizontal lines
                ctx.beginPath();
                ctx.moveTo(0, i * cellSize);
                ctx.lineTo(boardSize, i * cellSize);
                ctx.stroke();
            }

            // Draw numbers
            ctx.font = '36px Arial';
            ctx.textAlign = 'center';
            ctx.textBaseline = 'middle';
            for (let row = 0; row < 9; row++) {
                for (let col = 0; col < 9; col++) {
                    const value = board[row][col];
                    if (value !== 0) {
                        ctx.fillText(value.toString(), col * cellSize + cellSize / 2, row * cellSize + cellSize / 2);
                    }
                }
            }
        }
    }
</script>

<div class="game-wrapper">
    <div id="game" class="game">
        <canvas bind:this={canvas} width="450" height="450" style="width: 450px; height: 450px;"></canvas>
    </div>
</div>

