import React from "react";
import Image from "next/image";
import sample from "../../public/sample1.jpg";
import { Button } from "../ui";
const welcome = () => {
  return (
    <div className="flex flex-col py-2 gap-4 lg:gap-5">
      <h1 className="text-[50px] font-body font-bold bg-gradient-to-r from-[#2A2D4B] to-[#FFD700] bg-clip-text text-transparent">
        Welcome to FlowerWork!
      </h1>
      <h2 className="sm:md:block lg:hidden italic text-2xl">
        Your All in one Project Mangament Platform!
      </h2>
      <h2 className="hidden font-sans font-semibold lg:block italic text-2xl">
        Your All-in-one
        <br /> Project Mangament Platform!
      </h2>
      <div className="flex flex-col lg:flex-row lg:gap-x-12">
        <Image
          src={sample}
          alt="sample-image"
          className="rounded-3xl lg:w-1/3 md:w-full object-cover"
        />
        <div className="flex flex-col gap-4 text-justify text-[16px] py-3 font-body">
          <span>
            At FlowerWork, we're transforming the way you manage your projects.
            Whether you're a regular person looking for work for a quick
            project, a content creator, or a social media influencer who needs
            regular contract work, our platform is tailored to support your
            unique needs.
            <br />
            With AI-driven guidance, smartcontracts, and advanced quality
            control, FlowerWork helps you easily create, manage, and execute
            your projects from start to finish.
            <br />
          </span>
          <ul className="list-disc">
            <li>
              <strong>Need to find the perfect consultant?</strong>
              <span className="italic">We’ve got you covered.</span>
            </li>
            <li>
              <strong>Looking to streamline your workflow?</strong>
              <span className="italic">
                Our AI insights will keep you on track.
              </span>
            </li>
            <li>
              <strong>Ready to elevate your online presence?</strong>
              <span className="italic">
                FlowerWork ensures your projects are seamless, efficient, and
                successful.
              </span>
            </li>
          </ul>
          <strong className="text-2xl italic font-semibold">
            Join us and watch your work blossom with FlowerWork!
          </strong>
          <Button
            backgroundColor={"#ffd700"}
            hoverColor={"#181b3b"}
            text={"Make an account"}
            width={219}
            hoverwidth={270}
            height={61}
            padding={[18, 33]}
          />
        </div>
      </div>
    </div>
  );
};

export default welcome;
