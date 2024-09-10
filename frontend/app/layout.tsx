import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { ListItem, Navbar, NavBarProps } from "@/components/layout/Navbar";
import { UserIcon } from "lucide-react";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "FlowerWork",
  description: "A place where you can finsih your work",
};

let navbarlist: ListItem[] = [
  {
    title: "Login",
    link: "/login",
    icon: <UserIcon />,
  },
  {
    title: "Sign up",
    link: "/signup",
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
