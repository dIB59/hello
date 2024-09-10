import React from "react";
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
} from "../ui/navigation-menu";
import { NavigationMenuTrigger } from "@radix-ui/react-navigation-menu";

type NavbarProps = {
  list: string[];
};

export const Navbar: React.FC<NavbarProps> = ({ list }) => {
  return (
    <>
      <NavigationMenu>
        <NavigationMenuList>
          <NavigationMenuItem>
            <NavigationMenuLink>Menu</NavigationMenuLink>
          </NavigationMenuItem>
          {list.map((item) => (
            <NavigationMenuItem key={item}>
              <NavigationMenuLink>{item}</NavigationMenuLink>
            </NavigationMenuItem>
          ))}
        </NavigationMenuList>
      </NavigationMenu>
    </>
  );
};
