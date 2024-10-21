import React from "react";
import Link from "next/link";
import Image from "next/image";
import sample from "../../public/sample1.jpg";
import { CardSection } from ".";

const hero = () => {
  return (
    <div className="flex flex-col lg:py-6 lg:gap-10 ">
      {/* // Section 1 */}
      <div className="flex lg:flex-row gap-x-8">
        <div className="flex flex-col w-full gap-4 px-2 lg:px-0 ">
          <h1 className="text-4xl">
            What does{" "}
            <strong className="bg-gradient-to-r from-darkblue to-gold bg-clip-text text-transparent font-body font-bold">
              FlowerWork
            </strong>{" "}
            do?
          </h1>
          <div className="text-xl">
            <ul className="list-disc pl-5">
              <li className="italic font-semibold mt-4">
                Create and manage projects
                <ul className="ml-6 list-disc">
                  <li className="italic font-semibold mt-4">
                    Find the perfect consultant for your work
                    <ul className="ml-6 list-disc">
                      <li className="italic font-semibold mt-4">
                        Ensures deliverables are high quality by using advanced blockchain technology
                      </li>
                    </ul>
                  </li>
                </ul>
              </li>
            </ul>
          </div>
          <span className="text-xl mb-4">
            Read more about our services on our{" "}
            <Link href="/">
              <strong className="italic underline">services page</strong>
            </Link>{" "}
          </span>
        </div>
        <Image
          src={sample}
          alt="sample-image"
          className="hidden lg:block lg:w-1/3 md:w-1/2 object-cover h-auto rounded-3xl"
        />
      </div>
      {/* // Section 2 */}
      <CardSection />

      {/* // Section 3 */}
      <div className="flex flex-col lg:flex-row gap-5 py-3 lg:py-4">
        <Image
          src={sample}
          alt="sample-image"
          className="lg:w-1/3 md:w-full object-cover h-auto rounded-3xl"
        />
        <div className="flex flex-col justify-center gap-4 lg:w-full text-right">
          <span className="text-3xl font-bold font-body">
            <strong className="bg-gradient-to-r from-darkblue to-gold bg-clip-text text-transparent font-bold">
              FlowerWork
            </strong>{" "}
            cutting edge <br className="hidden lg:block" />
            technologies
          </span>
          <p className="text-xl">
            Explore how Flowerwork uses AI and blockchain technology to innovate
            tomorrow’s way of managing projects
          </p>
        </div>
      </div>

      {/* Section 4 */}
      <div className="flex flex-col py-2 gap-4 lg:flex-row lg:gap-4">
        <div className="flex flex-col gap-y-2 items-center lg:order-2 lg:gap-y-2 sm:w-full lg:w-8/12">
          <Image
            src={sample}
            alt="sample-image"
            className=" rounded-3xl w-full h-auto"
          />
          <div className="flex flex-row gap-2">
            <Image
              src={sample}
              alt="sample-image"
              className="w-1/2 h-auto rounded-3xl"
            />
            <Image
              src={sample}
              alt="sample-image"
              className="w-1/2 h-auto rounded-3xl"
            />
          </div>
        </div>
        <div className="flex flex-col lg:order-1 gap-4 w-full justify-center font-body ">
          <span className="text-3xl ">
            <span className="bg-gradient-to-r from-darkblue to-gold bg-clip-text text-transparent font-bold">
              FlowerWork
            </span>{" "}
            is at the top in the field,{" "}
            <strong className="italic font-bold">believe our users!</strong>{" "}
          </span>
          <p className="italic text-xl">
            How have our users liked our product?Find out from these inspiring
            testimonials
          </p>
        </div>
      </div>

      {/* Section 5 */}
      <div className="flex flex-col py-2 gap-4 lg:flex-row lg:py-4 lg:gap-4 ">
        <div className="flex gap-2 lg:gap-4 lg:w-full">
          <Image
            src={sample}
            alt="sample-image"
            className=" rounded-3xl w-1/2 "
          />
          <Image
            src={sample}
            alt="sample-image"
            className=" rounded-3xl w-1/2"
          />
        </div>
        <div className="flex flex-col gap-4 text-right w-full">
          <h1 className="text-4xl text-black">
            The squad making it all possible{" "}
            <span className="bg-gradient-to-r from-darkblue to-gold bg-clip-text text-transparent font-bold">
              FlowerWork
            </span>
          </h1>
          <p className="italic">Meet the team making the dream work</p>
        </div>
      </div>
    </div>
  );
};

export default hero;
