import React from "react";
import Image from "next/image";
import { Button } from "@/components/ui/button";
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuList,
} from "@/components/ui/navigation-menu";
import logo from "../../public/nav-logo.png";
import { CircleUser } from "lucide-react"; 

const Navbar = () => {
  return (
    <div className="w-full h-[92px] gap-2 lg:py-2">
      <div className="flex w-full h-[72px] justify-between items-center bg-darkblue lg:rounded-xl lg:px-2">
        <Image className="w-[69px] h-[72px]" src={logo} alt="logo" />

        {/* Navigation Menu */}
        <NavigationMenu className="flex items-center">
          <NavigationMenuList className="flex gap-4">
            {/* Log in Button with User Icon */}
            <NavigationMenuItem>
              <Button className="bg-transparent text-white border border-white w-[219px] h-[61px] rounded-full hover:bg-gold hover:text-black hover:border-black font-bold">
                <CircleUser className="mr-2 h-8 w-8" /> 
                Log in
              </Button>
            </NavigationMenuItem>

            {/* Sign up Button */}
            <NavigationMenuItem>
              <Button className="bg-transparent text-white border border-white w-[219px] h-[61px] rounded-full hover:bg-gold hover:text-black hover:border-black font-bold">
                Sign up
              </Button>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>
      </div>
    </div>
  );
};

export default Navbar;
