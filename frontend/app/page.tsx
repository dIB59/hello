import Image from "next/image";
import { Footer, Hero, Navbar, Welcome } from "./components";
import sample from "./public/sample1.jpg";

import AIAgentCreate from "./components/popup/AIAgentCreate";

export default function Home() {
  return (
    <div className="flex flex-col justify-center items-center lg:gap-4 lg:px-14 lg:py-5 md:gap-4 md:px-14 md:py-5">
      <Navbar />
      <div className="flex flex-col px-6 py-4 gap-10 lg:rounded-3xl lg:px-10 lg:gap-8 ">
        <Image
          src={sample}
          alt="sample-image"
          className="hidden lg:block md:block w-full rounded-3xl"
        />
        <Welcome />
        <Hero />
        <div className="hidden lg:inline-block md:inline-block items-center justify-center w-full">
          <div>
            <AIAgentCreate />
          </div>

          <Footer />
        </div>
      </div>
      <div className="lg:hidden md:hidden inline-block w-full">
        <Footer />
      </div>
    </div>
    // <div className="flex flex-col justify-center items-center p-9">
    //   <LogIn/>
    // </div>
  );
}
