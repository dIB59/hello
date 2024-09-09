import React from 'react'
import { Card } from '../ui'
const cardSecrion = () => {
  return (
    <div className='flex flex-col gap-4 py-4 lg:py-12 lg:gap-14 items-center'>
      <h1 className='text-4xl text-black'>
        We offer Multiple Subsribtion Plans!
      </h1>
      <div className='flex gap-3 lg:gap-7 flex-wrap w-full justify-evenly'>
        <Card cardType="Basic" rate={10} benefits={["You get this",'And also this','And more features!','How incredible']} link={""}/>
        <Card cardType="Standard" rate={15} benefits={["You get this",'And also this','And more features!','How incredible']}  link={""}/>
        <Card cardType="Premium" rate={30} benefits={["You get this",'And also this','And more features!','How incredible']}  link={""}/> 
      </div>
      
    </div>
  )
}

export default cardSecrion
