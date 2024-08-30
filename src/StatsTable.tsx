import { useState } from "react";
import { PlayerStats } from "./PlayerStats";
import StatsTableRow from "./StatsTableRow";

interface StatsTableProps {
    playerStats: PlayerStats[];
}

function formatLabels(label: string): string {
    if (label === "username") {
        return "Username";
    } else if (label === "level") {
        return "Level";
    } else if (label === "wins") {
        return "Wins";
    } else if (label === "winrate") {
        return "WinRate";
    } else if (label === "fkdr") {
        return "FKDR";
    } else if (label === "finalkills") {
        return "Finals";
    } else if (label === "rank") {
        return "Rank";
    } else {
        return "";
    }
}

export default function StatsTable({ playerStats }: Readonly<StatsTableProps>) {
    const [sortKey, setSortKey] = useState<keyof PlayerStats>("username");
    const [sortDirection, setSortDirection] = useState<"asc" | "desc">("asc");
    const sortedPlayerStats = [...playerStats];

    if (playerStats.length === 0) {
        return <></>;
    }

    const ranks = ["MVP++", "MVP+", "MVP", "VIP+", "VIP", "Non"];

    sortedPlayerStats.sort((a, b) => {
        if (sortDirection === "desc") {
            const temp = a;
            a = b;
            b = temp;
        }

        try {
            const aNum = parseInt(a[sortKey]);
            const bNum = parseInt(b[sortKey]);

            if (aNum < bNum) {
                return 1;
            } else if (aNum > bNum) {
                return -1;
            }
        } catch (_) {}

        if (sortKey === "rank") {
            return ranks.indexOf(a[sortKey]) - ranks.indexOf(b[sortKey]);
        } else if (sortKey === "username") {
            const aWords = a[sortKey].split(" ");
            const bWords = b[sortKey].split(" ");
            const aLastWord = aWords[aWords.length - 1];
            const bLastWord = bWords[bWords.length - 1];

            if (aLastWord < bLastWord) {
                return 1;
            }
            return -1;
        } else if (a[sortKey] < b[sortKey]) {
            return 1;
        } else if (a[sortKey] > b[sortKey]) {
            return -1;
        } else {
            return 0;
        }
    });

    return (
        <table>
            <thead>
                <tr>
                    {Object.keys(playerStats[0]).map((key) => {
                        if (["rank", "plus_color"].includes(key)) {
                            return "";
                        }
                        return (
                            <th
                                key={key}
                                onClick={() => {
                                    if (sortKey === key) {
                                        setSortDirection(sortDirection === "asc" ? "desc" : "asc");
                                    }
                                    setSortKey(key as keyof PlayerStats);
                                }}
                            >
                                {formatLabels(key)}
                            </th>
                        );
                    })}
                </tr>
            </thead>

            <tbody>
                {sortedPlayerStats.map((player) => (
                    <StatsTableRow key="username" playerStats={player} />
                ))}
            </tbody>
        </table>
    );
}
