import { Credentials, NewUser } from "../types/middleware";

export const signUP = async (newUser: NewUser): Promise<void> => {
    try {
        const res = await fetch("http://localhost:3000/sign_up",{
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(newUser),
        });
        if (!res.ok) {
            throw new Error("sign up error");
        }
    } catch {
        throw new Error("singup error");
    }
}

export const signIn = async (credential: Credentials): Promise<void> => {
    try {
        const res = await fetch("http://localhost:3000/sign_in",{
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify(credential),
            });
        if (!res.ok) {
            throw new Error("sign error");
        }
    } catch {
        throw new Error("unexpected error");
    }
}

export const signOut = async () => {  
    try {
        await fetch("http://localhost:3000/sign_out", {
            method: "GET",
            credentials: "include",
        });
    } catch {
       throw new Error("サインアウトできませんでした");
    }
}
