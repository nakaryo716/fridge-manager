import { SessionInfo } from "../types/middleware";

export enum SessionValue {
  Some,
  None,
}

export const isSession = async (): Promise<SessionValue> => {
  try {
    const response = await fetch("http://localhost:3000/is_session", {
      method: "GET",
      credentials: "include",
    });

    if (!response.ok) {
      return SessionValue.None;
    }

    const responseJson: SessionInfo = await response.json();
    console.log("セッション情報:", responseJson);

    return SessionValue.Some;
  } catch (error) {
    console.error("Fetch中に予期せぬエラーが発生しました。", error);
    return SessionValue.None;
  }
};
