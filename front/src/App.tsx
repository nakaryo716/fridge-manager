import { useEffect, useState } from "react";
import { InputItem } from "./components/InputItem"
import { NewFoodPayload, TrackedFood } from "./types/itemType";
import { deleteFoodApi, getAllFoodsApi, postFoodApi, updateFoodApi } from "./api/callApi";

export const App = () => {
  const [foods, setFoods] = useState<TrackedFood[]>([]);

  const onSubmitHandle = async (payload: NewFoodPayload) => {
    await postFoodApi(payload);

    const modifiedFoods = await getAllFoodsApi();
    setFoods(modifiedFoods);
    console.log(foods);
  };

  const onUpdateHandle = async (payload: TrackedFood) => {
    await updateFoodApi(payload);

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
    </>
  );
};