export type TtsId =
  | "aivoice"
  | "voiceroid"
  | "coefont"
  | "voicevox"
  | "softalk"
  | "talqu"
  | "voicepeak"
  | "cevioai";

export type TtsProfile = {
  accent: string;
  encoding: "UTF-8" | "Shift_JIS" | "UTF-16LE";
  id: TtsId;
  name: string;
};

export type OutputFormat = "srt" | "fcpxml";

export type GenerationStatus = {
  kind: "idle" | "running" | "success" | "error";
  message: string;
};
