"use client"
import React, {useCallback, useEffect, useRef, useState} from 'react';

interface SudokuGridProps {
    board: number[][];
    setBoard: React.Dispatch<React.SetStateAction<number[][]>>;
    solvedBoard: number[][]
}

const SudokuGrid: React.FC<SudokuGridProps> = ({board, setBoard, solvedBoard}) => {
        const canvasRef = useRef<HTMLCanvasElement>(null);
        const [selectedCell, setSelectedCell] = useState<{ row: number; col: number } | null>(null);
        const [errorCount, setErrorCount] = useState<number>(0);

        const handleClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
            const canvas = canvasRef.current;
            if (!canvas) {
                return
            }
            const rect = canvas.getBoundingClientRect();
            const x = event.clientX - rect.left;
            const y = event.clientY - rect.top;
            const cellSize = 50;

            const row = Math.floor(y / cellSize);
            const col = Math.floor(x / cellSize);
            setSelectedCell({row, col});  // Use state updater function
        };


        const handleKeyDown = useCallback((event: KeyboardEvent) => {
            //no cell is selected
            if (!selectedCell) return;

            //cell is already occupied so placing there would be a mistake
            if (board[selectedCell.row][selectedCell.col]) {
                return;
            }

            const value = parseInt(event.key, 10);
            if (value >= 1 && value <= 9) {
                //check if valid
                if (value !== solvedBoard[selectedCell.row][selectedCell.col]) {
                    setErrorCount((prev) => prev + 1);
                    return;
                }

                const newBoard = [...board];
                newBoard[selectedCell.row][selectedCell.col] = value;
                setBoard(newBoard);
            }
        }, [selectedCell, board]);


        useEffect(() => {
            window.addEventListener('keydown', handleKeyDown);
            return () => {
                window.removeEventListener('keydown', handleKeyDown);
            };
        }, [handleKeyDown]);


        useEffect(() => {
                const canvas = canvasRef.current;
                if (!canvas) {
                    return
                }
                const dpr = window.devicePixelRatio || 1;
                const rect = canvas.getBoundingClientRect();
                canvas.width = rect.width * dpr;
                canvas.height = rect.height * dpr;

                const ctx = canvas.getContext('2d')!;
                ctx.scale(dpr, dpr);

                const cellSize = 50;
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
                if (!selectedCell) {
                    return;
                }
                //highlight cells with same value
                const selectedValue = board[selectedCell.row][selectedCell.col];
                if (selectedValue !== 0) {
                    ctx.fillStyle = 'rgba(100,91,97,0.3)';  // Red color
                    for (let row = 0; row < 9; row++) {
                        for (let col = 0; col < 9; col++) {
                            if (board[row][col] === selectedValue && (row !== selectedCell.row || col !== selectedCell.col)) {
                                ctx.fillRect(
                                    col * cellSize,
                                    row * cellSize,
                                    cellSize,
                                    cellSize
                                );
                            }
                        }
                    }
                }

                //highlight all non blocking cells
                ctx.fillStyle = 'rgba(0, 128, 255, 0.15)';
                // Start a new path
                ctx.beginPath();

                // Highlight cell
                ctx.rect(
                    selectedCell.col * cellSize,
                    selectedCell.row * cellSize,
                    cellSize,
                    cellSize
                );

                // Highlight row
                ctx.rect(
                    0,
                    selectedCell.row * cellSize,
                    boardSize,
                    cellSize
                );

                // Highlight column
                ctx.rect(
                    selectedCell.col * cellSize,
                    0,
                    cellSize,
                    boardSize
                );

                // Highlight block
                const blockStartRow = Math.floor(selectedCell.row / 3) * 3;
                const blockStartCol = Math.floor(selectedCell.col / 3) * 3;
                ctx.rect(
                    blockStartCol * cellSize,
                    blockStartRow * cellSize,
                    3 * cellSize,
                    3 * cellSize
                );
                // Fill the entire path at once
                ctx.fill();
                ctx.fillStyle = 'rgba(0, 128, 255, 0.3)';
                ctx.fillRect(
                    selectedCell.col * cellSize,
                    selectedCell.row * cellSize,
                    cellSize,
                    cellSize
                );


            }, [board, selectedCell]
        )


        return (
            <div className="game-wrapper">

                <div>
                    Error Count: {errorCount}
                </div>
                <div id="game" className="game">
                    <canvas
                        ref={canvasRef}
                        width="450"
                        height="450"
                        style={{width: '450px', height: '450px'}}
                        onClick={handleClick}
                    />
                </div>
            </div>
        );
    }
;

export default SudokuGrid;
