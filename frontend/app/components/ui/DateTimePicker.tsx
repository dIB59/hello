"use client";
import * as React from "react"
import { useState } from "react";
import  Calendar  from "../../../components/ui/calendar";

export function CalendarDemo({ onDateSelect }: { onDateSelect: (date: Date | undefined) => void }) {
  const [date, setDate] = React.useState<Date | undefined>(new Date())
  const handleDateChange = (selectedDate: Date | undefined) => {
    setDate(selectedDate);
    onDateSelect(selectedDate);
  };
  return (
    <Calendar
      mode="single"
      selected={date}
      onSelect={handleDateChange}
      className="rounded-md border"
    />
  );
}

const DateTimePicker = () => {
  const [startDate, setStartDate] = useState<Date | undefined>();
  const [dueDate, setDueDate] = useState<Date | undefined>();
  const [selectedTime, setSelectedTime] = useState<string>("");
  const [isSelectingStartDate, setIsSelectingStartDate] = useState<boolean>(true); 

  const times = [
    "10:00", "10:30", "11:00", "11:30", "12:00",
    "12:30", "13:00", "13:30", "14:00", "14:30",
    "15:00", "15:30", "16:00", "16:30", "17:00",
    "17:30", "18:00", "18:30", "19:00", "19:30",
    "20:00", "20:30", "21:00", "21:30", "22:00",
  ];

  const handleDateSelect = (date: Date | undefined) => {
    if (!date) return;
    const correctedDate = new Date(Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()));
    if (isSelectingStartDate) {
      setStartDate(correctedDate);
    } else {
      setDueDate(correctedDate);
    }
    setIsSelectingStartDate(!isSelectingStartDate);
  };

  const handleTimeSelect = (time: string) => {
    setSelectedTime(time);
  };

  return (
    <div className="flex justify-between space-x-8">
      <div className="calendar-section w-1/2">
        <CalendarDemo onDateSelect={handleDateSelect} />
      </div>

      {/* Time Section */}
      <div className="time-section w-1/3 flex flex-col items-center max-h-[300px] overflow-y-auto border rounded-md p-2">
        <h2 className="text-lg font-semibold">Time</h2>
        <div className="time-list flex flex-col mt-4">
          {times.map((time) => (
            <button
              key={time}
              className={`mb-2 p-2 border rounded ${
                selectedTime === time ? "bg-blue-500 text-white" : "bg-gray-100"
              }`}
              onClick={() => handleTimeSelect(time)}
            >
              {time}
            </button>
          ))}
        </div>
      </div>

      {/* Start Date & Due Date & Hour & reminder Section */}
      <div className="w-1/3 space-y-4">
        {/* Startdate */}
        <div className="flex flex-col items-start space-x-2">
          <div className="flex items-center space-x-2"> 
            <input type="checkbox" className="form-checkbox" />

            <div className="flex items-center space-x-4"> 
              <div className="flex flex-col">
                <label className="text-lg font-semibold">Startdate:</label>
                <input
                  type="text"
                  value={startDate ? startDate.toISOString().split('T')[0] : ""}
                  readOnly
                  className="border p-2 rounded w-36"
                  placeholder="DD/MM/YYYY"
                />
              </div>
            </div>
          </div>
        </div>

        {/* Due Date and Hour */}
        <div className="flex flex-col items-start space-x-2">
          <div className="flex items-center space-x-2"> 
            <input type="checkbox" className="form-checkbox" />

            <div className="flex items-center space-x-4"> 
              {/* Due Date */}
              <div className="flex flex-col">
                <label className="text-lg font-semibold">Due date:</label>
                <input
                  type="text"
                  value={dueDate ? dueDate.toISOString().split('T')[0] : ""}
                  readOnly
                  className="border p-2 rounded w-36"
                  placeholder="DD/MM/YYYY"
                />
              </div>
              {/* Hour*/}
              <div className="flex flex-col">
                <label className="text-lg font-semibold">Hour:</label>
                <input
                  type="text"
                  value={selectedTime}
                  readOnly
                  className="border p-2 rounded w-20"
                  placeholder="13:30"
                />
              </div>
            </div>
          </div>
        </div>

        <div className="reminder-section">
          <h2 className="text-lg font-semibold">Set due date reminder</h2>
          <select className="border p-2 rounded mt-2  w-full">
            <option value="none">None</option>
            <option value="At-time-of-due-date">At time of due date</option>
            <option value="5-min">5 minutes before</option>
            <option value="10-min">10 minutes before</option>
            <option value="15-min">15 minutes before</option>
            <option value="30-min">30 minutes before</option>
            <option value="1-hour">1 hour before</option>
            <option value="1-day">1 day before</option>
          </select>
        </div>

        <div className="actions flex flex-col space-y-2 justify-between mt-4 ">
          <button className="p-2 w-full bg-white-500 text-black rounded border border-black mr-2 hover:bg-customBlue hover:text-white" >
            Save
          </button>
          <button className="p-2 w-full bg-white-500 text-black rounded border border-black hover:bg-customBlue hover:text-white">
            Remove
          </button>
        </div>
      </div>
    </div>
  );
};

export default DateTimePicker;