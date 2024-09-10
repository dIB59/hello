"use client";

import * as React from "react";
import Link from "next/link";

import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
} from "@/components/ui/navigation-menu";
import Image from "next/image";

export type ListItem = {
  title: string;
  link: string;
  icon?: React.JSX.Element;
};

export type NavBarProps = {
  list: ListItem[];
};

export const Navbar: React.FC<NavBarProps> = ({ list }) => {
  return (
    <header className="p-3 m-2 bg-secondary rounded-xl">
      
      <NavigationMenu className="flex justify-between items-center max-w-full">
   
        <NavigationMenuItem className="">
          <Image
            src="/logo.png"
            alt="Company Logo"
            className="logo-img"
            width={"60"}
            height={"40"}
          />
        </NavigationMenuItem>

        <NavigationMenuList className="flex space-x-4">
          {list.map((item, index) => (
            <NavigationMenuItem key={index}>
              <Link href={item.link} legacyBehavior passHref>
                <NavigationMenuLink className="text-white bg-secondary rounded-2xl p-4 border px-8">
                  {item.icon && <UserIcon />}
                  {item.title}
                </NavigationMenuLink>
              </Link>
            </NavigationMenuItem>
          ))}
        </NavigationMenuList>
      </NavigationMenu>
    </header>
  );
};

const UserIcon = (): React.JSX.Element => (
  <svg
    width="29"
    height="29"
    viewBox="0 0 29 29"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    className="w-6 h-6 inline-block mr-2"
  >
    <path
      d="M6.04375 21.5125C7.2125 20.6187 8.51875 19.9141 9.9625 19.3984C11.4063 18.8828 12.9187 18.625 14.5 18.625C16.0813 18.625 17.5938 18.8828 19.0375 19.3984C20.4813 19.9141 21.7875 20.6187 22.9562 21.5125C23.7583 20.5729 24.3828 19.5073 24.8297 18.3156C25.2766 17.124 25.5 15.8521 25.5 14.5C25.5 11.4521 24.4286 8.85677 22.2859 6.71406C20.1432 4.57135 17.5479 3.5 14.5 3.5C11.4521 3.5 8.85677 4.57135 6.71406 6.71406C4.57135 8.85677 3.5 11.4521 3.5 14.5C3.5 15.8521 3.72344 17.124 4.17031 18.3156C4.61719 19.5073 5.24167 20.5729 6.04375 21.5125ZM14.5 15.875C13.1479 15.875 12.0078 15.4109 11.0797 14.4828C10.1516 13.5547 9.6875 12.4146 9.6875 11.0625C9.6875 9.71042 10.1516 8.57031 11.0797 7.64219C12.0078 6.71406 13.1479 6.25 14.5 6.25C15.8521 6.25 16.9922 6.71406 17.9203 7.64219C18.8484 8.57031 19.3125 9.71042 19.3125 11.0625C19.3125 12.4146 18.8484 13.5547 17.9203 14.4828C16.9922 15.4109 15.8521 15.875 14.5 15.875ZM14.5 28.25C12.5979 28.25 10.8104 27.8891 9.1375 27.1672C7.46458 26.4453 6.00938 25.4656 4.77187 24.2281C3.53437 22.9906 2.55469 21.5354 1.83281 19.8625C1.11094 18.1896 0.75 16.4021 0.75 14.5C0.75 12.5979 1.11094 10.8104 1.83281 9.1375C2.55469 7.46458 3.53437 6.00938 4.77187 4.77187C6.00938 3.53437 7.46458 2.55469 9.1375 1.83281C10.8104 1.11094 12.5979 0.75 14.5 0.75C16.4021 0.75 18.1896 1.11094 19.8625 1.83281C21.5354 2.55469 22.9906 3.53437 24.2281 4.77187C25.4656 6.00938 26.4453 7.46458 27.1672 9.1375C27.8891 10.8104 28.25 12.5979 28.25 14.5C28.25 16.4021 27.8891 18.1896 27.1672 19.8625C26.4453 21.5354 25.4656 22.9906 24.2281 24.2281C22.9906 25.4656 21.5354 26.4453 19.8625 27.1672C18.1896 27.8891 16.4021 28.25 14.5 28.25ZM14.5 25.5C15.7146 25.5 16.8604 25.3224 17.9375 24.9672C19.0146 24.612 20 24.1021 20.8938 23.4375C20 22.7729 19.0146 22.263 17.9375 21.9078C16.8604 21.5526 15.7146 21.375 14.5 21.375C13.2854 21.375 12.1396 21.5526 11.0625 21.9078C9.98542 22.263 9 22.7729 8.10625 23.4375C9 24.1021 9.98542 24.612 11.0625 24.9672C12.1396 25.3224 13.2854 25.5 14.5 25.5ZM14.5 13.125C15.0958 13.125 15.5885 12.9302 15.9781 12.5406C16.3677 12.151 16.5625 11.6583 16.5625 11.0625C16.5625 10.4667 16.3677 9.97396 15.9781 9.58438C15.5885 9.19479 15.0958 9 14.5 9C13.9042 9 13.4115 9.19479 13.0219 9.58438C12.6323 9.97396 12.4375 10.4667 12.4375 11.0625C12.4375 11.6583 12.6323 12.151 13.0219 12.5406C13.4115 12.9302 13.9042 13.125 14.5 13.125Z"
      fill="white"
    />
  </svg>
);
