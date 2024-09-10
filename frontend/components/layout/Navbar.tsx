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
        <NavigationMenuItem>
          <NavigationMenuTrigger>Item One</NavigationMenuTrigger>
          <NavigationMenuContent>
            <NavigationMenuLink>Link</NavigationMenuLink>
          </NavigationMenuContent>
        </NavigationMenuItem>
      </NavigationMenu>
    </>
  );
};
