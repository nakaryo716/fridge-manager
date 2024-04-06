export type TrackedFood = {
    id: number,
    foodName: string,
    expirationDate: Date,
    used: boolean,
};

export type NewFoodPayload = {
    foodName: string,
    expirationDate: Date,
};
