"use client"

import SudokuGrid from "@/components/SudokuGrid";
import {useQuery, useQueryClient} from "@tanstack/react-query";
import {useEffect, useState} from "react";

const stringToBoard = (str: string): number[][] => {
    const arr = Array.from(str).map(Number);
    return Array.from({length: 9}, (_, i) => arr.slice(i * 9, (i + 1) * 9));
};

const fetchDefaultBoard = async (): Promise<string> => {
    const res = await fetch(process.env.NEXT_PUBLIC_API_URL +'/random');
    if (!res.ok) {
        throw new Error('Network response was not ok');
    }
    const data = await res.json();
    return data["board"];
};

const fetchSolvedBoard = async (board: string): Promise<string> => {
    const res = await fetch(process.env.NEXT_PUBLIC_API_URL + '/sudoku?board=' + board);
    if (!res.ok) {
        throw new Error('Network response was not ok');
    }
    const data = await res.json();
    return data["board"];
};


const IndexPage = () => {
    const queryClient = useQueryClient();
    const [refreshKey, setRefreshKey] = useState(0);
    const [board, setBoard] = useState<number[][]>([]);
    const [solvedBoard, setSolvedBoard] = useState<number[][]>([]);

    const generateNewBoard = () => {
        setRefreshKey(prevKey => prevKey + 1); // Increment the refreshKey to force a new fetch
    };

    const {data: defaultBoard, isSuccess: isDefaultBoardSuccess} = useQuery(
        {
            queryKey: ['board', refreshKey],
            queryFn: fetchDefaultBoard,

        }
    );

    useEffect(() => {
        if (isDefaultBoardSuccess && defaultBoard) {
            setBoard(stringToBoard(defaultBoard));
            queryClient.prefetchQuery(['solvedBoard', defaultBoard], () => fetchSolvedBoard(defaultBoard));
        }
    }, [isDefaultBoardSuccess, defaultBoard, queryClient, refreshKey]); // Added refreshKey to dependencies

    const {data: fetchedSolvedBoard, isSuccess: isSolvedBoardSuccess} = useQuery(
        ['solvedBoard', defaultBoard],
        () => fetchSolvedBoard(defaultBoard!),
        {enabled: !!defaultBoard}
    );

    useEffect(() => {
        if (isSolvedBoardSuccess && fetchedSolvedBoard) {
            setSolvedBoard(stringToBoard(fetchedSolvedBoard));
        }
    }, [isSolvedBoardSuccess, fetchedSolvedBoard]);

    const solveBoard = () => {
        if (fetchedSolvedBoard) {
            setBoard(stringToBoard(fetchedSolvedBoard));
        }
    };


    if (!isDefaultBoardSuccess || !board.length) {
        return <div>Loading...</div>;
    }

    return (
        <div className="flex flex-col justify-center items-center">
            <div className="flex justify-between mb-4">
                <button className={"mx-2"} onClick={solveBoard}>Solve Board</button>
                <button className={"mx-2"} onClick={generateNewBoard}>New Board</button>
            </div>
            <SudokuGrid board={board} setBoard={setBoard} solvedBoard={solvedBoard}/>
        </div>

    );
};


export default IndexPage;