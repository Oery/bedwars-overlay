type Rank = "YOUTUBER" | "MVP++" | "MVP+" | "MVP" | "VIP+" | "VIP" | "Non";

type PlusColor =
    | "RED"
    | "GOLD"
    | "GREEN"
    | "YELLOW"
    | "LIGHT_PURPLE"
    | "WHITE"
    | "BLUE"
    | "DARK_GREEN"
    | "DARK_RED"
    | "DARK_AQUA"
    | "DARK_PURPLE"
    | "DARK_GRAY"
    | "BLACK"
    | "DARK_BLUE";

type Prestige =
    | "STONE"
    | "IRON"
    | "GOLD"
    | "DIAMOND"
    | "EMERALD"
    | "SAPPHIRE"
    | "RUBY"
    | "CRYSTAL"
    | "OPAL"
    | "AMETHYST"
    | "RAINBOW";

interface PlayerStats {
    level: string;
    username: string;
    wins: string;
    winrate: string;
    fkdr: string;
    finalkills: string;
    rank: Rank;
    plus_color: PlusColor;
}

interface PlayerChange {
    username: string;
    count: number;
    total: number;
    has_joined: boolean;
}

export { PlayerStats, Rank, Prestige, PlayerChange, PlusColor };
