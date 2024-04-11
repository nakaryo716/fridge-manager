// APIとの型をそろえるためにスネークケースになっている
// expiration_dateはAPI側のDate型がタイムゾーンに対応してないためstring型にしている
export type TrackedFood = {
    id: number,
    name: string,
    expiration_date: string,
    used: boolean,
};

export type NewFoodPayload = {
    name: string,
    expiration_date: string,
};

export type UpdateFoodPayload = {
    name: string,
    expiration_date: string,
    used: boolean,
};
