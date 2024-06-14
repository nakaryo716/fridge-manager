import { NewUser, User } from "../types/middleware";

export const signUP = async (newUser: NewUser): Promise<User> => {
    const res = await fetch("http://localhost:3000/sign_up",{
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(newUser),
    });

    const responseJson: User = await res.json();
    return responseJson;
}

export const signIn = async (credential: Credential): Promise<User> => {
    const res = await fetch("http://localhost:3000/sign_up",{
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify(credential),
    });

    const responseJson: User = await res.json();
    return responseJson;
}
