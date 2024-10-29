"use client"

import produce from 'immer';
import { create } from 'zustand';
import { devtools, persist, subscribeWithSelector } from 'zustand/middleware';

const store = (set : any) => ({
    tokenSelected: '',
    priceCalaulated: 0,
    address : "",
    prices : [],
    timestamps : [],
    changeToken : (token : string) => set({tokenSelected: token}),
    changePrice : (price : number) => set({priceCalaulated: price}),
    changeAddress : (address : string) => set({address: address}),
    changePrices : (prices : any) => set({prices: prices}), 
    changeTimestamps : (timestamps : any) => set({timestamps: timestamps})
})

const log = (config : any) => (set : any, get : any, api : any) =>
  config(
    (...args : any) => {
      console.log(args);
      set(...args);
    },
    get,
    api
  );

  

export const useStore = create(
    subscribeWithSelector(log(persist(devtools(store), { name: 'store' })))
  );
  