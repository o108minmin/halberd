import { useEffect, useState } from "react";

export function usePersistentState<T>(
  key: string,
  initialValue: T,
  validate: (value: unknown) => value is T,
) {
  const [value, setValue] = useState<T>(() => {
    try {
      const storedValue = window.localStorage.getItem(key);
      if (storedValue === null) return initialValue;

      const parsedValue: unknown = JSON.parse(storedValue);
      return validate(parsedValue) ? parsedValue : initialValue;
    } catch {
      return initialValue;
    }
  });

  useEffect(() => {
    try {
      window.localStorage.setItem(key, JSON.stringify(value));
    } catch {
      // 設定の保存に失敗しても字幕生成は継続できるため、処理を止めない。
    }
  }, [key, value]);

  return [value, setValue] as const;
}
