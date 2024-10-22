import React from "react";
import Link from "next/link";
import { Button } from "../ui";
const login = () => {
  return (
    <div className="flex flex-col sm:w-full md:w-9/12 lg:w-8/12 rounded-3xl py-10 px-3 border-2 border-black items-center">
      <div className="flex flex-col gap-5 py-6 items-center">
        <div className="flex flex-col">
          <span className="text-[34px] font-body bg-gradient-to-r from-darkblue to-gold bg-clip-text text-transparent font-bold">
            Welcome back,
          <span className="text-[34px] font-body bg-gradient-to-r from-darkblue to-gold bg-clip-text text-transparent font-bold">
            {" "}superstar!
          </span>
          </span>
          <span className="italic">
            What will <b>you</b> be creating today?
          </span>
        </div>
        <div className="flex flex-col items-center w-full">
          <form action="#" className="flex flex-col gap-6 w-full">
            <div className="flex flex-col gap-1">
              <label className="font-body font-semibold" htmlFor="email">Email address:</label>
              <input
                type="email"
                name="email"
                id="email"
                required
                className="border-[2px] rounded-[20px] border-darkblue h-12 px-2 lg:h-[65px]"
              />
            </div>
            <div className="flex flex-col gap-1">
              <label className="font-body font-semibold" htmlFor="password">Password:</label>
              <input
                type="password"
                name="password"
                id="password"
                required
                className="border-[2px] rounded-[20px] border-darkblue h-12 px-2 lg:h-[65px]"
              />
              <Link className="underline italic" href="#">Forget your password?</Link>
            </div>
            <div className="flex gap-2">
              <input type="checkbox" name="RememberMe" id="RememberMe" />
              <label htmlFor="RememberMe">Remeber Me?</label>
            </div>
          </form>
        </div>
        <div className="flex flex-col lg:flex-row items-center justify-between w-full">
        <Button text="Login" width={200} height={60} backgroundColor="#2A2D4B" hoverwidth={220} padding={[14, 60]} textColor="white"/>
        <span >Dont have an account?</span>
        <Button text="Create an Account" width={200} height={60} backgroundColor="#2A2D4B" hoverwidth={220} padding={[14, 60]} textColor="white"/>
        </div>
      </div>
    </div>
  );
};

export default login;
