import { TrackedFood, NewFoodPayload, UpdateFoodPayload } from "../types/itemType";

export const postFoodApi = async (payload: NewFoodPayload) => {
    const res = await fetch("http://100.64.1.18:3000/fridge", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    });

    if (!res) {
        throw new Error("Could not post data");
    }

    const responseJson: TrackedFood = await res.json();
    return responseJson;
};

export const getAllFoodsApi = async () => {
    const res = await fetch("http://100.64.1.18:3000/fridge");

    if (!res) {
        throw new Error("Could not get items");
    }

    const responseJson: TrackedFood[] = await res.json();
    return responseJson;
};

export const updateFoodApi = async (id: number, payload: UpdateFoodPayload) => {
    const res = await fetch(`http://100.64.1.18:3000/fridge/${id}`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    });

    if (!res) {
        throw new Error("Could not update item");
    }

    const responseJson: TrackedFood = await res.json();
    return responseJson;
}

export const deleteFoodApi = async (id: number) => {
    const res = await fetch(`http://100.64.1.18:3000/fridge/${id}`, {
        method: "DELETE",
    });

    if (!res) {
        throw new Error("Could not delete item");
    }
}
