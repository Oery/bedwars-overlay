import { Home, Settings, History, Database, LogOut, UsersRound } from "lucide-react";

export default function Sidebar() {
    return (
        <aside>
            <nav>
                <ul>
                    <li>
                        <Home />
                    </li>
                    <li>
                        <History />
                    </li>
                    <li>
                        <UsersRound />
                    </li>
                    <li>
                        <Database />
                    </li>
                </ul>
            </nav>

            <nav>
                <ul>
                    <li>
                        <Settings />
                    </li>
                    <li>
                        <LogOut />
                    </li>
                </ul>
            </nav>
        </aside>
    );
}
