"use client"


import { create } from 'zustand';
import { devtools, persist, subscribeWithSelector } from 'zustand/middleware';

const store = (set : any) => ({
    tokenSelected: '',
    priceCalaulated: 0,
    address : "",
    changeToken : (token : string) => set({tokenSelected: token}),
    changePrice : (price : number) => set({priceCalaulated: price}),
    changeAddress : (address : string) => set({address: address}),
})

const tokenStore = (set : any) => ({
  primaryToken: '',
  referenceToken: '',
  changePrimaryToken : (token : string) => set({primaryToken: token}),
  changeReferenceToken : (token : string) => set({referenceToken: token}),
})

const priceStore = (set : any) => ({
  prices : [],
  dates : [],
  addPrice : (price : string) => set((state : any) => ({
    prices : [price, ...state.prices]
  })),
  removePrice: () =>
    set((state : any) => ({
      prices: state.prices.slice(0, -1), // Remove the last price
    })),
  addDates : (date : string) => set((state : any) => {
    dates : [date, ...state.dates]
  }),
  removeDate: () =>
    set((state : any) => ({
      dates: state.dates.slice(0, -1), // Remove the last price
    })),
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

  export const useTokenStore = create(
    subscribeWithSelector(log(persist(devtools(tokenStore), { name: 'tokenStore' })))
  );
  
  export const usePriceStore = create(
    subscribeWithSelector(log(persist(devtools(priceStore), { name: 'priceStore' })))
  );