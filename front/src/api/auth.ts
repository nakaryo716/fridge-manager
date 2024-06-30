import { Credentials, NewUser } from "../types/middleware";

export const signUP = async (newUser: NewUser): Promise<Response> => {
    try {
        const res = await fetch("http://localhost:3000/sign_up",{
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(newUser),
        });
        return res;
    } catch {
        throw new Error("sign up error");
    }
}

export const signIn = async (credential: Credentials): Promise<Response> => {
    try {
        const res = await fetch("http://localhost:3000/sign_in",{
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: "include",
                body: JSON.stringify(credential),
            });
        return res;
    } catch {
        throw new Error("error");
        
    }
}

export const signOut = async () => {  
    try {
        const res = await fetch("http://localhost:3000/sign_out", {
            method: "GET",
            credentials: "include",
        });
        // エラーを投げさせる
        // catchに移動させる
        if(!res.ok) {
            throw new Error();
        }
    } catch {
       throw new Error("サインアウトできませんでした");
    }
}
