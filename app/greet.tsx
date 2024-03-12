'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

export default function Greet() {
   const [greeting, setGreeting] = useState('');

   useEffect(() => {
      invoke<string>('greet')
         .then(result => setGreeting(result))
         .catch(console.error)
   }, [])

   console.log(greeting)
   // Necessary because we will have to use Greet as a component later.
   return <>{greeting}</>;
}