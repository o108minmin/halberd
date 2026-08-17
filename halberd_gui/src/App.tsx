import { useState } from "react";
import { Box } from "@mui/material";
import { AppShell } from "./components/AppShell";
import { SubtitleGenerator } from "./features/subtitle-generator/SubtitleGenerator";

export type MenuId = "subtitle-generator" | "history" | "settings";

function App() {
  const [activeMenu, setActiveMenu] = useState<MenuId>("subtitle-generator");

  return (
    <AppShell activeMenu={activeMenu} onMenuChange={setActiveMenu}>
      {activeMenu === "subtitle-generator" ? (
        <SubtitleGenerator />
      ) : (
        <Box />
      )}
    </AppShell>
  );
}

export default App;
