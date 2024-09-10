import React from "react";
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
} from "../ui/navigation-menu";
import { NavigationMenuTrigger } from "@radix-ui/react-navigation-menu";
import { Button } from "../ui/button";
import { FileX } from "lucide-react";

type NavbarProps = {
  list: string[];
};

export const Navbar: React.FC<NavbarProps> = ({ list }) => {
  return (
    <>
      <div className={"flex justify-end bg-sky-950 p-3 m-2" }>
        <NavigationMenu>
          <NavigationMenuList>
            <Button> Sign In</Button>

            <Button> Register</Button>
          </NavigationMenuList>
        </NavigationMenu>
      </div>
    </>
  );
};
