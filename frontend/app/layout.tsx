import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { ListItem, Navbar, NavBarProps } from "@/components/layout/Navbar";
import { Button } from "@/components/ui/button";
import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarTrigger,
} from "@/components/ui/menubar";
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuTrigger,
} from "@/components/ui/navigation-menu";
import { list } from "postcss";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "FlowerWork",
  description: "A place where you can finsih your work",
};

let navbarlist: ListItem[] = [
  {
    title: "Home",
    link: "/",
  },
  {
    title: "About",
    link: "/about",
  },
  {
    title: "Contact",
    link: "/contact",
  },
];

let props = {
  list: navbarlist,
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Navbar list={navbarlist} />

        {children}
      </body>
    </html>
  );
}
