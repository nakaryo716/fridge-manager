export const isSession = async (): Promise<Response> => {
  try {
    const response = await fetch("http://localhost:3000/is_session", {
      method: "GET",
      credentials: "include",
    });
    return response;
  } catch (error) {
    throw new Error("server err");
  }
};
