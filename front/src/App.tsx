import { useEffect, useState } from "react";
import { InputItem } from "./components/InputItem"
import { NewFoodPayload, TrackedFood, UpdateFoodPayload } from "./types/itemType";
import { deleteFoodApi, getAllFoodsApi, postFoodApi, updateFoodApi } from "./api/callApi";
import { ItemList } from "./components/ItemList";

export const App = () => {
  const [foods, setFoods] = useState<TrackedFood[]>([]);

  const onSubmitHandle = async (payload: NewFoodPayload) => {
    await postFoodApi(payload);

    const modifiedFoods = await getAllFoodsApi();
    setFoods(modifiedFoods);
  };

  const onUpdateHandle = async (id: number, payload: UpdateFoodPayload) => {
    await updateFoodApi(id, payload);

    const modifiedFoods = await getAllFoodsApi();
    setFoods(modifiedFoods);
  };

  const onDeleteHandle = async (id: number) => {
    await deleteFoodApi(id);

    const modifiedFoods = await getAllFoodsApi();
    setFoods(modifiedFoods);
  };

  // 初回バインド時にデータ取得
  useEffect(() => {
    (async () => {
      const getFoods = await getAllFoodsApi();
      setFoods(getFoods);
    })();
  }, []);

  return(
    <>
      <h1 style={{textAlign: "center"}}>賞味・消費期限マネージャ</h1>
      <InputItem onSubmitHandle={onSubmitHandle} />
      <ItemList foods={foods} onUpdateHandle={onUpdateHandle} onDeleteHandle={onDeleteHandle}></ItemList>
    </>
  );
};
