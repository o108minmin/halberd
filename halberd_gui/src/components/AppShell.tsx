import type { ReactNode } from "react";
import HistoryRoundedIcon from "@mui/icons-material/HistoryRounded";
import SettingsRoundedIcon from "@mui/icons-material/SettingsRounded";
import SubtitlesRoundedIcon from "@mui/icons-material/SubtitlesRounded";
import {
  Box,
  ButtonBase,
  Chip,
  Stack,
  Typography,
} from "@mui/material";
import type { MenuId } from "../App";
import { halberdLogo } from "../assets";

type AppShellProps = {
  activeMenu: MenuId;
  children: ReactNode;
  onMenuChange: (menu: MenuId) => void;
};

const menuItems: Array<{
  id: MenuId;
  label: string;
  description: string;
  icon: ReactNode;
  disabled?: boolean;
}> = [
  {
    id: "subtitle-generator",
    label: "字幕生成",
    description: "TTSから字幕を書き出す",
    icon: <SubtitlesRoundedIcon />,
  },
  {
    id: "history",
    label: "生成履歴",
    description: "近日追加予定",
    icon: <HistoryRoundedIcon />,
    disabled: true,
  },
  {
    id: "settings",
    label: "設定",
    description: "近日追加予定",
    icon: <SettingsRoundedIcon />,
    disabled: true,
  },
];

export function AppShell({ activeMenu, children, onMenuChange }: AppShellProps) {
  return (
    <Box className="app-shell">
      <Box component="aside" className="sidebar">
        <Stack className="brand" direction="row" spacing={1.5} alignItems="center">
          <Box
            alt="halberd"
            className="brand__mark"
            component="img"
            src={halberdLogo}
          />
          <Box>
            <Typography className="brand__name">halberd</Typography>
            <Typography className="brand__caption">Subtitle Studio</Typography>
          </Box>
        </Stack>

        <Typography className="sidebar__eyebrow">WORKSPACE</Typography>
        <Stack component="nav" spacing={1}>
          {menuItems.map((item) => {
            const selected = item.id === activeMenu;
            return (
              <ButtonBase
                key={item.id}
                className={`nav-item${selected ? " nav-item--active" : ""}`}
                disabled={item.disabled}
                onClick={() => onMenuChange(item.id)}
              >
                <Box className="nav-item__icon">{item.icon}</Box>
                <Box className="nav-item__copy">
                  <Typography className="nav-item__label">{item.label}</Typography>
                  <Typography className="nav-item__description">
                    {item.description}
                  </Typography>
                </Box>
                {item.disabled && <Chip label="Soon" size="small" />}
              </ButtonBase>
            );
          })}
        </Stack>

        <Box className="sidebar__footer">
          <Box className="sidebar__status-dot" />
          <Box>
            <Typography className="sidebar__status-title">halberd core</Typography>
            <Typography className="sidebar__status-caption">Ready to generate</Typography>
          </Box>
        </Box>
      </Box>

      <Box component="main" className="main-content">
        {children}
      </Box>
    </Box>
  );
}
