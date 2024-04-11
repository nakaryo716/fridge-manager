import { Button, Checkbox, Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow } from "@mui/material";
import { TrackedFood, UpdateFoodPayload } from "../types/itemType";

type Props = {
    foods: TrackedFood[],
    onUpdateHandle: (id: number, payload: UpdateFoodPayload) => Promise<void>,
    onDeleteHandle: (id: number) => Promise<void>,
};

export const ItemList = (props: Props) => {
    const {foods, onUpdateHandle, onDeleteHandle} = props;

    const onClickCheckBox = async (Updates: TrackedFood) => {
        const {id, name, expiration_date, used} = Updates;

        const payload: UpdateFoodPayload = {
            name,
            expiration_date,
            used: !used
        };  
        await onUpdateHandle(id, payload);
    };

    const onClickDelete = async (id: number) => {
        await onDeleteHandle(id);
    };
    
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
                                <TableRow key={food.id}>
                                    <TableCell align="left"><Checkbox onChange={() => onClickCheckBox(food)} checked={food.used}/>{food.name}</TableCell>
                                    <TableCell align="left">{food.expiration_date}</TableCell>
                                    <TableCell align="left">
                                        <Button variant="contained" sx={{backgroundColor: "mediumturquoise", "&:hover":{backgroundColor: "darkcyan"}}}>編集</Button>
                                    </TableCell>
                                    <TableCell align="left">
                                        <Button variant="contained" sx={{backgroundColor: "pink", "&:hover":{backgroundColor: "hotpink"}}} onClick={() => onClickDelete(food.id)}>削除</Button>
                                    </TableCell>
                                </TableRow>
                            )
                        })}
                    </TableBody>
                </Table>
            </TableContainer>
        </div>
    );
};
