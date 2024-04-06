export type Item = {
    id: number,
    name: string,
    expirationDate: Date,
    used: boolean,
};

export type newItemPayload = {
    name: string,
    expirationDate: Date,
};
