export type NewUser = {
    user_name: string,
    mail: string,
    password: string
}

export type Credentials = {
    mail: string,
    password: string,
}

export type User = {
    user_id: number,
    user_name: string,
    mail: string,
    password: string,
}

export type SessionInfo = {
    user_id: number,
}