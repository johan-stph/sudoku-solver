"use client"

import Sudoku from "@/components/Sudoku";
import {QueryClient} from "@tanstack/query-core";
import {QueryClientProvider} from "@tanstack/react-query";

export default function Home() {
    const queryClient = new QueryClient();
    return (
        <main className="flex min-h-screen flex-col items-center justify-between p-24">
            <QueryClientProvider client={queryClient}>
                <Sudoku/>
            </QueryClientProvider>
        </main>
    )
}
