import { Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow } from "@mui/material";
import { TrackedFood, UpdateFoodPayload } from "../types/itemType";
import { FoodItem } from "./FoodItem";

type Props = {
    foods: TrackedFood[],
    onUpdateHandle: (id: number, payload: UpdateFoodPayload) => Promise<void>,
    onDeleteHandle: (id: number) => Promise<void>,
};

export const ItemList = (props: Props) => {
    const {foods, onUpdateHandle, onDeleteHandle} = props;

    return (
        <div>
            <TableContainer component={Paper} sx={{width: 1200, margin: "auto"}}>
                <Table sx={{ minWidth: 650 }} aria-label="simple table">
                    <TableHead>
                        <TableRow>
                            <TableCell>食品</TableCell>
                            <TableCell align="left">期限</TableCell>
                            <TableCell align="left">編集</TableCell>
                            <TableCell align="left">削除</TableCell>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {foods.map((food) => {
                            return(
                                <FoodItem key={food.food_id} food={food} onUpdateHandle={onUpdateHandle} onDeleteHandle={onDeleteHandle} />
                            )
                        })}
                    </TableBody>
                </Table>
            </TableContainer>
        </div>
    );
};
