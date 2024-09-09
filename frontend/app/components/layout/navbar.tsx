import React from "react";
import Image from "next/image";
import { Button } from "../ui";
import logo from "../../public/nav-logo.png";

const navbar = () => {
  return (
    <div className="w-full mx-auto h-[92px] gap-2 lg:py-2">
      <div className="flex w-full h-[72px] justify-between items-center bg-[#2A2D4B] rounded-xl lg:gap-2 px-2 ">
        <Image className="w-[69px] h-[72px] " src={logo} alt="logo" />

        <div className="flex px-4 gap-4">
          <div className="hidden md:block lg:block">
            <Button
              backgroundColor="#2A2D4B"
              hoverColor="#181b3b"
              hoverwidth={250}
              text="Log in"
              textColor="white"
              width={219}
              height={61}
              padding={[14, 60]}
            />
            <Button
              backgroundColor="#2A2D4B"
              hoverColor="#181b3b"
              hoverwidth={250}
              text="Sign up"
              textColor="white"
              width={219}
              height={61}
              padding={[14, 60]}
            />
          </div>
          <div className="block md:hidden lg:hidden">
            <Button
              backgroundColor="#2A2D4B"
              hoverColor="#181b3b"
              hoverwidth={150}
              text="Sign up"
              textColor="white"
              width={120}
              height={61}
              padding={[14, 60]}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default navbar;
