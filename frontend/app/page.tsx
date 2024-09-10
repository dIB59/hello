/* eslint-disable react/no-unescaped-entities */
import { Button } from "@/components/ui/button";
import Image from "next/image";

export default function Home() {
  return (
    <body className="bg-gray-100">
      {/* Hero Section */}
      <section className="text-center py-16 bg-white">
        <h1 className="text-5xl font-bold">Welcome to FlowerWork!</h1>
        <h2 className="text-2xl mt-4">Your All-in-One Project Management Platform</h2>
      </section>

      {/* Introduction Section */}
      <section className="py-16 px-6">
        <p className="text-lg leading-relaxed">
        At FlowerWork, we're transforming the way you manage your projects. Whether you're a regular person looking for work for a quick project, a content creator, or a social media influencer who needs regular contract work, our platform is tailored to support your unique needs. 
        With AI-driven guidance, smart contracts, and advanced quality control, FlowerWork helps you easily create, manage, and execute your projects from start to finish.
        </p>

        <ul className="list-disc pl-4 mt-2">
          <li>Need to find the perfect consultant? We’ve got you covered.</li>
          <li>Looking to streamline your workflow? Our AI insights will keep you on track. </li>
          <li>Ready to elevate your online presence? FlowerWork ensures your projects are seamless, efficient, and successful.
          </li>
        </ul>

    
      </section>

      {/* Subscription Plans Section */}
      <section className="py-16 px-6 bg-gray-50">
        <h2 className="text-3xl font-bold text-center mb-8">We offer multiple subscription plans!</h2>
        <div className="flex justify-center space-x-6">
          {/* Basic Plan */}
          <div className="border p-4 rounded-lg bg-white shadow-lg">
            <h3 className="text-xl font-bold">Basic - $10</h3>
            <ul className="list-disc pl-4 mt-2">
              <li>You get this</li>
              <li>And also this</li>
              <li>And more features!</li>
            </ul>
            <Button className="mt-4 bg-yellow-500 text-white px-4 py-2 rounded">Choose Plan</Button>
          </div>
          {/* Standard Plan */}
          <div className="border p-4 rounded-lg bg-white shadow-lg">
            <h3 className="text-xl font-bold">Standard - $15</h3>
            <ul className="list-disc pl-4 mt-2">
              <li>You get this</li>
              <li>And also this</li>
              <li>And more features!</li>
            </ul>
            <Button className="mt-4 bg-yellow-500 text-white px-4 py-2 rounded">Choose Plan</Button>
          </div>
          {/* Premium Plan */}
          <div className="border p-4 rounded-lg bg-white shadow-lg">
            <h3 className="text-xl font-bold">Premium - $30</h3>
            <ul className="list-disc pl-4 mt-2">
              <li>You get this</li>
              <li>And also this</li>
              <li>And more features!</li>
            </ul>
            <Button className="mt-4 bg-yellow-500 text-white px-4 py-2 rounded">Choose Plan</Button>
          </div>
        </div>
      </section>

      {/* Technology Section */}
      <section className="py-16 px-6">
        <h2 className="text-3xl font-bold text-center">FlowerWork 🫶 Cutting Edge Technology</h2>
        <p className="text-center mt-4">Explore how Flowerwork uses AI and blockchain technology to innovate tomorrow’s way of managing projects.</p>
      </section>

      {/* Testimonials Section */}
      <section className="py-16 px-6 bg-gray-50">
        <h2 className="text-3xl font-bold text-center">How have our users liked our product?</h2>
        <p className="text-center mt-4">Find out from these inspiring testimonials!</p>
      </section>

      {/* Footer Section */}
      <footer className="text-center py-6 bg-gray-800 text-white">
        <p>&copy; 2024 FlowerWork - All Rights Reserved</p>
      </footer>
    </body>
  );
}
