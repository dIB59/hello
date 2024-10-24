"use client";
import React from 'react';

type ButtonProps = {
  backgroundColor: string;
  hoverColor?: string;
  text: string;
  textColor?: "white" | "black";
  width: number;
  hoverwidth: number;
  height: number;
  padding: [number, number];
};

const Button = ({
  backgroundColor,
  hoverColor,
  text,
  textColor,
  width,
  hoverwidth,
  height,
  padding,
}: ButtonProps) => {
  return (
    <button
      type="button"
      className={`
        ${textColor === "white" ? "text-white" : "text-black"} 
        font-bold
        p-[${padding[0]}px] px-[${padding[1]}px]
        rounded-[40px] 
        border 
        transition-all duration-300  // Smooth transition for hover effect
        
      `}
      style={{
        backgroundColor: backgroundColor,
        width: `${width}px`,
        height: `${height}px`,
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.backgroundColor = hoverColor || backgroundColor;
        e.currentTarget.style.width = `${hoverwidth}px`;
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.backgroundColor = backgroundColor;
        e.currentTarget.style.width = `${width}px`;
      }}
    >
      {text}
    </button>
  );
};

export default Button;
