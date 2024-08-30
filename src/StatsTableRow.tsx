import { PlayerStats, Prestige } from "./PlayerStats";
import { v4 as uuidv4 } from "uuid";
import Plus from "./Plus";
import React from "react";

interface StatsTableRowProps {
    playerStats: PlayerStats;
}

function getPrestigeByLevel(level: number): Prestige {
    if (level < 100) {
        return "STONE";
    } else if (level < 200) {
        return "IRON";
    } else if (level < 300) {
        return "GOLD";
    } else if (level < 400) {
        return "DIAMOND";
    } else if (level < 500) {
        return "EMERALD";
    } else if (level < 600) {
        return "SAPPHIRE";
    } else if (level < 700) {
        return "RUBY";
    } else if (level < 800) {
        return "CRYSTAL";
    } else if (level < 900) {
        return "OPAL";
    } else if (level < 1000) {
        return "AMETHYST";
    } else {
        return "RAINBOW";
    }
}

function getColorByFKDR(fkdr: number): string {
    if (fkdr < 1) {
        return "STONE";
    } else if (fkdr < 2) {
        return "IRON";
    } else if (fkdr < 3) {
        return "GOLD";
    } else if (fkdr < 5) {
        return "DIAMOND";
    } else if (fkdr < 6) {
        return "EMERALD";
    } else if (fkdr < 7) {
        return "SAPPHIRE";
    } else if (fkdr < 8) {
        return "RUBY";
    } else if (fkdr < 9) {
        return "CRYSTAL";
    } else if (fkdr < 13) {
        return "OPAL";
    } else if (fkdr < 17) {
        return "AMETHYST";
    } else {
        return "RAINBOW";
    }
}

function getColorByWinRate(winRate: number): string {
    if (winRate < 30) {
        return "STONE";
    } else if (winRate < 40) {
        return "IRON";
    } else if (winRate < 50) {
        return "GOLD";
    } else if (winRate < 60) {
        return "DIAMOND";
    } else if (winRate < 70) {
        return "EMERALD";
    } else if (winRate < 75) {
        return "SAPPHIRE";
    } else if (winRate < 80) {
        return "RUBY";
    } else if (winRate < 85) {
        return "CRYSTAL";
    } else if (winRate < 90) {
        return "OPAL";
    } else if (winRate < 95) {
        return "AMETHYST";
    } else {
        return "RAINBOW";
    }
}

function getColorByWins(wins: number): string {
    if (wins < 1000) {
        return "STONE";
    } else if (wins < 2000) {
        return "IRON";
    } else if (wins < 3000) {
        return "GOLD";
    } else if (wins < 4000) {
        return "DIAMOND";
    } else if (wins < 5000) {
        return "EMERALD";
    } else if (wins < 6000) {
        return "SAPPHIRE";
    } else if (wins < 7000) {
        return "RUBY";
    } else if (wins < 8000) {
        return "CRYSTAL";
    } else if (wins < 9000) {
        return "OPAL";
    } else if (wins < 10000) {
        return "AMETHYST";
    } else {
        return "RAINBOW";
    }
}

function getColorByFinals(finals: number): string {
    if (finals < 1000) {
        return "STONE";
    } else if (finals < 2000) {
        return "IRON";
    } else if (finals < 5000) {
        return "GOLD";
    } else if (finals < 8000) {
        return "DIAMOND";
    } else if (finals < 10000) {
        return "EMERALD";
    } else if (finals < 15000) {
        return "SAPPHIRE";
    } else if (finals < 20000) {
        return "RUBY";
    } else if (finals < 25000) {
        return "CRYSTAL";
    } else if (finals < 30000) {
        return "OPAL";
    } else if (finals < 40000) {
        return "AMETHYST";
    } else {
        return "RAINBOW";
    }
}

function getClasses(key: string, value: string, playerStats: PlayerStats): string {
    if (key === "username") {
        if (playerStats.rank === "YOUTUBER") {
            return "RED username";
        }
        return `${playerStats.rank.replace("+", "_PLUS").replace("+", "_PLUS")} username`;
    } else if (key === "rank") {
        return value.replace("+", "_PLUS").replace("+", "_PLUS");
    } else if (key === "level") {
        return getPrestigeByLevel(parseInt(value));
    } else if (key === "fkdr") {
        return `${getColorByFKDR(parseFloat(value))} FKDR`;
    } else if (key === "winrate") {
        return `${getColorByWinRate(parseFloat(value.slice(0, 4)))} WLR`;
    } else if (key === "finalkills") {
        return `${getColorByFinals(parseInt(value.replace(".", "")))} FINALS`;
    } else if (key === "wins") {
        return `${getColorByWins(parseInt(value.replace(".", "")))} WINS`;
    } else {
        return "";
    }
}

function processValue(key: string, value: string, playerStats: PlayerStats) {
    if (value === "Non") {
        return "";
    } else if (key === "username") {
        console.log(value, playerStats.rank);
        if (playerStats.rank === "Non") {
            return value;
        } else if (playerStats.rank === "YOUTUBER") {
            return (
                <>
                    {"["}
                    <span className="WHITE">YT</span>
                    {"] " + value}
                </>
            );
        } else {
            const parts = playerStats.rank.split("+");
            return (
                <>
                    {"["}
                    {parts.map((part: string, index: number) => (
                        <React.Fragment key={uuidv4()}>
                            {part}
                            {index < parts.length - 1 && <Plus color={playerStats.plus_color} />}
                        </React.Fragment>
                    ))}
                    {"] " + value}
                </>
            );
        }
    } else if (key === "finalkills") {
        return value.replace(/\B(?=(\d{3})+(?!\d))/g, ".");
    } else if (key === "wins") {
        return value.replace(/\B(?=(\d{3})+(?!\d))/g, ".");
    } else if (key === "winrate") {
        return value.slice(0, 4) + "%";
    } else if (key === "fkdr") {
        return value.slice(0, 4);
    } else {
        return value;
    }
}

export default function StatsTableRow({ playerStats }: Readonly<StatsTableRowProps>) {
    return (
        <tr>
            {Object.entries(playerStats).map(([key, value]) => {
                if (["rank", "plus_color"].includes(key)) {
                    return "";
                }

                return (
                    <td key={uuidv4()} className={getClasses(key, value, playerStats)}>
                        {processValue(key, value, playerStats)}
                    </td>
                );
            })}
        </tr>
    );
}
