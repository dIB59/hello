
import React from 'react'
import Link from 'next/link'
import Button from './button'


type Card={
  cardType:string,
  rate:number,
  benefits?:[string,string,string,string] | [string,string,string],
  link:string,
}

const card = ({cardType,rate,benefits,link}:Card) => {
  return (
    <div className='flex flex-col  bg-white w-[307px] h-[372px] rounded-[40px] py-7 gap-3 border-solid border-2 border-black shadow-xl'>
      <div className='flex flex-col gap-2 items-center py-6'>
        <div className='flex flex-row justify-between w-[257px]'>
          <h1 className='text-3xl font-bold items-center'>{cardType}</h1>
          <h1 className='text-3xl font-bold items-center'>${rate}</h1>
        </div>
        <ul className='list-disc px-6 '>
            {benefits.map((benefit)=>(<li key={benefit}>{benefit}</li>))}
        </ul>
        <Link className='underline italic text-gray-400' href={link}>Learn more about this</Link>
      </div>
      <div className='flex justify-center'>
      <Button backgroundColor={'#ffd700'} hoverColor={'#181b3b'} text={'Choose a plan'} width={180} hoverwidth={290} height={64}  padding={[18,33]}/>
      </div>
            
    </div>
  )
}

export default card
