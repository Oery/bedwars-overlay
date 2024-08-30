import { PlusColor } from "./PlayerStats";

interface PlusProps {
    color: PlusColor;
}

export default function Plus({ color }: PlusProps) {
    console.log("Creating Plus with color " + color);
    return <span className={color}>+</span>;
}
