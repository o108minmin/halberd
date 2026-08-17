import { createTheme } from "@mui/material/styles";

export const theme = createTheme({
  palette: {
    mode: "dark",
    primary: {
      main: "#7c6cff",
      light: "#a99cff",
      dark: "#5d4ee7",
    },
    background: {
      default: "#080b14",
      paper: "#111522",
    },
    text: {
      primary: "#f5f7ff",
      secondary: "#929bb0",
    },
  },
  shape: {
    borderRadius: 16,
  },
  typography: {
    fontFamily:
      '-apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans JP", "Hiragino Sans", "Yu Gothic UI", "Yu Gothic", Meiryo, sans-serif',
    body1: {
      lineHeight: 1.75,
    },
    body2: {
      lineHeight: 1.7,
    },
    button: {
      fontWeight: 700,
      letterSpacing: "0.015em",
      textTransform: "none",
    },
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          borderRadius: 12,
        },
      },
    },
    MuiChip: {
      styleOverrides: {
        root: {
          fontWeight: 700,
        },
      },
    },
    MuiMenu: {
      styleOverrides: {
        paper: {
          border: "1px solid rgba(255, 255, 255, 0.08)",
          backgroundImage: "none",
        },
      },
    },
  },
});
