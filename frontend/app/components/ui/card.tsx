import React from 'react';
import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card';

type CardProps = {
  cardType: string,
  rate: number,
  benefits: string[],
  link: string,
};

const PricingCard = ({ cardType, rate, benefits, link }: CardProps) => {
  return (
    <Card className="w-[307px] h-[372px] rounded-[40px] border-2 border-black shadow-custom">
      {/* Card Header */}
      <CardHeader className="text-center">
        <CardTitle className="flex justify-between items-center w-[257px] mx-auto">
          <div className="flex flex-row justify-between items-center w-full">
            <h3 className="text-3xl font-bold">{cardType}</h3>
            <p className="text-3xl font-bold">${rate}</p> 
          </div>
        </CardTitle>
      </CardHeader>

      {/* Card Content (Benefits) */}
      <CardContent className="flex flex-col items-center">
        <ul className="list-disc px-6 space-y-2">
          {benefits?.map((benefit) => (
            <li key={benefit} className="italic font-semibold">{benefit}</li>
          ))}
        </ul>

        {/* Link */}
        <div className="w-full flex justify-left mt-4">
          <Link className="underline italic text-gray-400" href={link}>
            Learn more about this
          </Link>
        </div>
      </CardContent>

      {/* Card Footer (Button) */}
      <CardFooter className="flex justify-center">
        <Button
          className="bg-gold hover:bg-[#181b3b] text-black hover:text-white font-bold w-[180px] h-[64px] rounded-full transition-all duration-300"
        >
          Choose a plan
        </Button>
      </CardFooter>
    </Card>
  );
};

export default PricingCard;
