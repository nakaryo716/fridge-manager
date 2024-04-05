import { Button, FormControl, InputLabel, MenuItem, Select, SelectChangeEvent, TextField } from "@mui/material"
import React, { useState } from "react";

export const InputItem = () => {
    // デフォルト値を設定するために現在の日付を取得
    const currentYMD = new Date();
    const currentYear = currentYMD.getFullYear();
    const currentMonth = currentYMD.getUTCMonth();
    const currentDate = currentYMD.getUTCDay();

    // セレクトバーに表示するための連番生成
    const sequentialYear = Array.from({length: 15}, (_, i) => (currentYear - 1) + i + 1);
    const sequentialMonth = Array.from({length: 12}, (_, i) => i + 1);
    const sequentialDate = Array.from({length: 31}, (_, i) => i + 1);

    const [todos, setTodos] = useState<string[]>([]);
    const [text, setText] = useState("");
    const [selectedYear, setSelectedYear] = useState(currentYear);
    const [selectedMonth, setSelectedMonth] = useState(currentMonth);
    const [selectedDate, setSelectedDate] = useState(currentDate);

    const followTextHandle = (e: React.ChangeEvent<HTMLInputElement>) => {
        setText(e.target.value);
        console.log(text);
    };

    const onSubmit = () => {
        setTodos([text, ...todos]);
        console.log(todos);
        console.log(selectedYear);
        console.log(selectedMonth);
        console.log(selectedDate);
    };
    
    const onChengeYear = (e: SelectChangeEvent<number>) => {
        setSelectedYear(e.target.value as number);
    };

    const onChengeMonth = (e: SelectChangeEvent<number>) => {
        setSelectedMonth(e.target.value as number);
    };

    const onChengeDate = (e: SelectChangeEvent<number>) => {
        setSelectedDate(e.target.value as number);
    };

    return(
        <>
            <div style={{textAlign: "center", margin: 40}}>
                <TextField sx={{width: 400}}id="filled-basic" label="商品名" variant="filled" onChange={(e:  React.ChangeEvent<HTMLInputElement>) =>followTextHandle(e)}/>
                <FormControl sx={{minWidth: 120, marginLeft: 2}}>
                    <InputLabel>Year</InputLabel>
                    <Select defaultValue={currentYear} onChange={(e) => onChengeYear(e)}>
                        {sequentialYear.map((year) => {
                            return(
                                <MenuItem value={year}>{year}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <FormControl sx={{minWidth: 120, marginLeft: 1}}>
                    <InputLabel>Month</InputLabel>
                    <Select defaultValue={currentMonth} onChange={(e) => onChengeMonth(e)}>
                        {sequentialMonth.map((month) => {
                            return(
                                <MenuItem value={month}>{month}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <FormControl sx={{minWidth: 120, marginLeft: 1}}>
                    <InputLabel>Date</InputLabel>
                    <Select defaultValue={currentDate} onChange={(e) => onChengeDate(e)}>
                        {sequentialDate.map((valueNum) => {
                            return(
                                <MenuItem value={valueNum}>{valueNum}</MenuItem>
                            );
                        })}
                    </Select>
                </FormControl>
                <Button size="large" variant="contained" sx={{height: 56, width: 180, marginLeft: 4}} onClick={() => onSubmit()}>追加</Button>
            </div>
        </>
    );
};
