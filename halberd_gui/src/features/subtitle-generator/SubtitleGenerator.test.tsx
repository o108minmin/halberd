import { ThemeProvider } from "@mui/material";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { theme } from "../../theme";
import { SubtitleGenerator } from "./SubtitleGenerator";

const mocks = vi.hoisted(() => ({
  homeDir: vi.fn(),
  invoke: vi.fn(),
  open: vi.fn(),
  revealItemInDir: vi.fn(),
  save: vi.fn(),
  writeText: vi.fn(),
}));

vi.mock("@tauri-apps/api/core", () => ({ invoke: mocks.invoke }));
vi.mock("@tauri-apps/api/path", () => ({ homeDir: mocks.homeDir }));
vi.mock("@tauri-apps/plugin-clipboard-manager", () => ({ writeText: mocks.writeText }));
vi.mock("@tauri-apps/plugin-dialog", () => ({ open: mocks.open, save: mocks.save }));
vi.mock("@tauri-apps/plugin-opener", () => ({ revealItemInDir: mocks.revealItemInDir }));

function renderGenerator() {
  return render(
    <ThemeProvider theme={theme}>
      <SubtitleGenerator />
    </ThemeProvider>,
  );
}

async function completeSettings() {
  const user = userEvent.setup();
  mocks.homeDir.mockResolvedValue("/Users/test");
  mocks.open.mockResolvedValue("/Users/test/voicevox");
  mocks.save.mockResolvedValue("/Users/test/voicevox.srt");

  await user.click(screen.getByRole("button", { name: "TTSプロファイル" }));
  await user.click(screen.getByRole("option", { name: /VOICEVOX/ }));
  await user.click(screen.getByRole("button", { name: "フォルダを選択" }));
  await user.click(screen.getByRole("button", { name: "保存先を選択" }));

  return user;
}

describe("SubtitleGenerator", () => {
  beforeEach(() => {
    window.localStorage.clear();
  });

  it("未入力の項目を表示し、生成ボタンを無効にする", () => {
    renderGenerator();

    expect(screen.getByRole("button", { name: "字幕を生成" })).toBeDisabled();
    expect(
      screen.getByText("TTSプロファイル・入力フォルダ・出力先を指定してください。"),
    ).toBeInTheDocument();
  });

  it("設定完了後に字幕を生成し、生成後アクションを実行できる", async () => {
    mocks.invoke.mockResolvedValue("保存しました: /Users/test/voicevox.srt");
    renderGenerator();
    const user = await completeSettings();

    const generateButton = screen.getByRole("button", { name: "字幕を生成" });
    expect(generateButton).toBeEnabled();
    expect(screen.getAllByText("設定済み")).toHaveLength(3);

    await user.click(generateButton);
    expect(await screen.findByText("字幕ファイルを生成しました")).toBeInTheDocument();
    expect(mocks.invoke).toHaveBeenCalledWith("halberd_run", {
      input: "/Users/test/voicevox",
      output: "/Users/test/voicevox.srt",
      tts: "voicevox",
    });

    await user.click(screen.getByRole("button", { name: "出力先を表示" }));
    expect(mocks.revealItemInDir).toHaveBeenCalledWith("/Users/test/voicevox.srt");

    await user.click(screen.getByRole("button", { name: "パスをコピー" }));
    expect(mocks.writeText).toHaveBeenCalledWith("/Users/test/voicevox.srt");

    await user.click(screen.getByRole("button", { name: "もう一度生成" }));
    await waitFor(() => expect(mocks.invoke).toHaveBeenCalledTimes(2));
  });

  it("保存したTTS・入力フォルダ・出力形式を復元する", () => {
    window.localStorage.setItem("halberd.subtitle-generator.tts", JSON.stringify("talqu"));
    window.localStorage.setItem(
      "halberd.subtitle-generator.input",
      JSON.stringify("/Users/test/talqu"),
    );
    window.localStorage.setItem("halberd.subtitle-generator.format", JSON.stringify("fcpxml"));

    renderGenerator();

    expect(screen.getByRole("button", { name: "TTSプロファイル" })).toHaveTextContent("TALQu");
    expect(screen.getByText("talqu")).toBeInTheDocument();
    expect(screen.getByRole("button", { name: /FCPXML/ })).toHaveAttribute("aria-pressed", "true");
  });
});
