import Image from "next/image";
import { Footer, Hero, Navbar, Welcome } from "./components";
import sample from "./public/sample1.jpg"; 
import bgImage from "./public/landing-page.jpg";

export default function Home() {
  return (
    <div
      className="min-h-screen w-full flex flex-col justify-center bg-no-repeat bg-cover"
      style={{ backgroundImage: `url(${bgImage.src})` }}
    >
      {/* Main Content Area */}
      <div className="flex flex-col lg:gap-4 lg:px-14 lg:py-5 md:gap-4 md:px-14 md:py-5 w-full">
        {/* Header Section */}
        <header>
          <Navbar />
        </header>

        {/* Main Section */}
        <main className="flex flex-col px-6 py-4 gap-10 lg:rounded-3xl lg:px-10 lg:gap-8 bg-white ">
          <div className="hidden md:block w-full">
            <Image
              src={sample}
              alt="sample-image"
              className="w-full rounded-3xl object-cover"
            />
          </div>
          <Welcome />
          <Hero />
        </main>

        {/* Footer Section */}
        <footer>
          <div className="hidden lg:inline-block md:inline-block items-center justify-center w-full">
            <Footer />
          </div>
          <div className="lg:hidden md:hidden inline-block w-full">
            <Footer />
          </div>
        </footer>
      </div>
    </div>
  );
}
