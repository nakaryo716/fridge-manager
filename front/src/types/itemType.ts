// APIとの型をそろえるためにスネークケースになっている
// expiration_dateはAPI側のDate型がタイムゾーンに対応してないためstring型にしている
export type TrackedFood = {
    food_id: number,
    food_name: string,
    expiration: string,
    used: boolean,
    user_id: string,
};

export type NewFoodPayload = {
    food_name: string,
    expiration: string,
};

export type UpdateFoodPayload = {
    food_name: string,
    expiration: string,
    used: boolean,
};
