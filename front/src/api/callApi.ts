import { Item, newItemPayload } from "../types/itemType";

export const postItems = async (payload: newItemPayload) => {
    const res = await fetch("http://localhost:3000/fridge", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload)
    });

    if (!res) {
        throw new Error("Could not post data");
    }

    const responseJson: Item = await res.json();
    return responseJson;
};

export const getAllItem = async () => {
    const res = await fetch("http://localhost:3000/fridge");

    if (!res) {
        throw new Error("Could not get items");
    }

    const responseJson: Item = await res.json();
    return responseJson;
};

export const updateItem = async (payload: Item) => {
    const {id, ...updateContents} = payload;

    const res = await fetch(`http://localhost:3000/fridge/${id}`, {
        method: "PUT",
        headers: {
            "Contents-Type": "application/json",
        },
        body: JSON.stringify(updateContents),
    });

    if (!res) {
        throw new Error("Could not update item");
    }

    const responseJson: Item = await res.json();
    return responseJson;
}

export const deleteItem = async (id: number) => {
    const res = await fetch(`http://localhost:3000/fridge/${id}`, {
        method: "DELETE",
    });

    if (!res) {
        throw new Error("Could not delete item");
    }
}
