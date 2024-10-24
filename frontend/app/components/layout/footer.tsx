import React from 'react'
import Link from 'next/link'

import DiscordIcon from '../icons/DiscordIcon';
import GitHubIcon from '../icons/GitHubIcon';
import FacebookIcon from '../icons/FacebookIcon';
import InstagramIcon from '../icons/InstagramIcon';

const footer = () => {
  return (
    <div className='flex flex-col w-full lg:flex-row bg-darkblue rounded-t-3xl gap-y-10 items-center p-4 lg:justify-around  md:rounded-3xl lg:rounded-3xl lg:px-11 lg:py-8'>
       <div className='grid grid-cols-3 gap-10 gap-x-[120px] text-white'>
          <Link className='Carrers' href=';'>Hello</Link>
          <Link className='underline' href=';'>Hello</Link>
          <Link className='underline' href=';'>Hello</Link>
          <Link className='underline' href=';'>Hello</Link>
          <Link className='underline' href=';'>Hello</Link>
          <Link className='underline' href=';'>Hello</Link>
          <Link className='underline' href=';'>Hello</Link>
          <Link className='underline' href=';'>Hello</Link>
          <Link className='underline' href=';'>Hello</Link>
        </div>
        <div className='flex lg:w-[270px] lg:gap-x-20 md:justify-evenly lg:justify-between'>
          <span className="[&>svg]:h-10 [&>svg]:w-10">
            <DiscordIcon />
          </span>
          <span className="[&>svg]:h-10 [&>svg]:w-10">
            <GitHubIcon />
          </span>
          <span className="[&>svg]:h-10 [&>svg]:w-10">
            <FacebookIcon />
          </span>
          <span className="[&>svg]:h-10 [&>svg]:w-10">
            <InstagramIcon />
          </span>
        </div>
    </div>
  )
}

export default footer;
