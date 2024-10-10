import React from 'react';
import PricingCard from '../ui/card'

const CardSection = () => {
  return (
    <div className='flex flex-col gap-4 py-4 lg:py-12 lg:gap-14 items-center'>
      <h1 className='text-4xl text-black'>
        We offer Multiple Subscription Plans!
      </h1>
      <div className='flex gap-3 lg:gap-7 flex-wrap w-full justify-center'>
        <PricingCard 
          cardType="Basic" 
          rate={10} 
          benefits={["You get this", "And also this", "And more features!", "How incredible"]} 
          link="" 
        />
        <PricingCard 
          cardType="Standard" 
          rate={15} 
          benefits={["You get this", "And also this", "And more features!", "How incredible"]} 
          link="" 
        />
        <PricingCard 
          cardType="Premium" 
          rate={30} 
          benefits={["You get this", "And also this", "And more features!", "How incredible"]} 
          link="" 
        />
      </div>
    </div>
  );
};

export default CardSection;
