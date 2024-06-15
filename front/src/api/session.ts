import { SessionInfo } from "../types/middleware";

export const isSession = async (): Promise<SessionInfo> => {
    const res = await fetch("http://localhost:3000/is_session", {
        method: "GET",
        credentials: "include",
    });

    const responseJson: SessionInfo = await res.json();
    return responseJson;
}
