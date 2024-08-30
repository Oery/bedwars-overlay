import { useEffect, useState } from "react";
import { PlayerChange, PlayerStats, PlusColor, Rank } from "./PlayerStats";
import StatsTable from "./StatsTable";
import { Event, listen } from "@tauri-apps/api/event";
import Sidebar from "./Sidebar";

function createRandomPlayerStats(): PlayerStats {
    const ranks: Rank[] = ["MVP++", "MVP+", "MVP", "VIP+", "VIP", "Non", "YOUTUBER"];
    const plus_colors: PlusColor[] = [
        "WHITE",
        "GOLD",
        "GREEN",
        "LIGHT_PURPLE",
        "BLUE",
        "YELLOW",
        "DARK_GREEN",
        "DARK_RED",
        "BLACK",
        "DARK_BLUE",
    ];

    return {
        rank: ranks[Math.floor(Math.random() * ranks.length)],
        username: "Player_" + Math.floor(Math.random() * 1000),
        level: Math.floor(Math.random() * 1099).toString(),
        fkdr: (Math.random() * 20).toFixed(2),
        finalkills: Math.floor(Math.random() * 100000).toString(),
        wins: Math.floor(Math.random() * 1000).toString(),
        winrate: (Math.random() * 100).toFixed(1) + "%",
        plus_color: plus_colors[Math.floor(Math.random() * plus_colors.length)],
    };
}

function createRandomPlayerStatsArray(): PlayerStats[] {
    const playerStats: PlayerStats[] = [];

    for (let i = 0; i < 16; i++) {
        playerStats.push(createRandomPlayerStats());
    }

    return playerStats;
}

function addPlayerStatsToArray(playerStats: PlayerStats[], username: string): PlayerStats[] {
    if (playerStats.some((playerStat) => playerStat.username === username)) {
        return playerStats;
    }

    const newPlayerStats: PlayerStats = {
        rank: "Non",
        username: username,
        level: "0",
        fkdr: "0.00",
        finalkills: "0",
        wins: "0",
        winrate: "0.0%",
        plus_color: "WHITE",
    };

    return [...playerStats, newPlayerStats];
}

function removePlayerStatsFromArray(playerStats: PlayerStats[], username: string): PlayerStats[] {
    return playerStats.filter((playerStat) => playerStat.username !== username);
}

function App() {
    const [playerStats, setPlayerStats] = useState<PlayerStats[]>(createRandomPlayerStatsArray());

    useEffect(() => {
        const unlistenClearPlayers = listen("clear-players", () => {
            console.log("clearing");
            setPlayerStats([]);
        });

        const unlistenAddPlayer = listen("add-player", (event: Event<PlayerChange>) => {
            console.log("adding", event.payload);
            setPlayerStats((prevPlayerStats) => addPlayerStatsToArray(prevPlayerStats, event.payload.username));
        });

        const unlistenUpdatePlayer = listen("update-player", (event: Event<PlayerStats>) => {
            console.log("updating", event.payload);
            setPlayerStats((prevPlayerStats) => {
                const playerStatsCopy = [...prevPlayerStats];
                const index = playerStatsCopy.findIndex((playerStat) => playerStat.username === event.payload.username);
                playerStatsCopy[index] = event.payload;
                return playerStatsCopy;
            });
        });

        const unlistenRemovePlayer = listen("remove-player", (event: Event<PlayerChange>) => {
            console.log("removing", event.payload);
            setPlayerStats((prevPlayerStats) => removePlayerStatsFromArray(prevPlayerStats, event.payload.username));
        });

        const unlistenSetPlayers = listen("set-players", (event: Event<string[]>) => {
            console.log("setting");
            setPlayerStats(
                event.payload.map((username) => {
                    return {
                        rank: "Non",
                        username: username,
                        level: "0",
                        fkdr: "0.00",
                        finalkills: "0",
                        wins: "0",
                        winrate: "0.0%",
                        plus_color: "WHITE",
                    };
                })
            );
        });

        return () => {
            unlistenClearPlayers.then((f) => f());
            unlistenAddPlayer.then((f) => f());
            unlistenRemovePlayer.then((f) => f());
            unlistenSetPlayers.then((f) => f());
            unlistenUpdatePlayer.then((f) => f());
        };
    }, []);

    return (
        <>
            <Sidebar />
            {/* <button
                onClick={() => setPlayerStats(createRandomPlayerStatsArray())}
            >
                Click to repopulate
            </button>
            <button
                onClick={() =>
                    setPlayerStats(addPlayerStatsToArray(playerStats, "test"))
                }
            >
                Click to add
            </button>
            <button
                onClick={() =>
                    setPlayerStats(
                        removePlayerStatsFromArray(playerStats, "test")
                    )
                }
            >
                Click to remove
            </button> */}
            <StatsTable playerStats={playerStats} />
        </>
    );
}

export default App;
