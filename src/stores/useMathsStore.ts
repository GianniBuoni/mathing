import { create } from "zustand";
export type MathsState = {
  fullPrice: number;
  jonPrice: number;
  paulPrice: number;
  jonPays: (price: number) => void;
  paulPays: (price: number) => void;
};

const useMathsStore = create<MathsState>((set) => ({
  fullPrice: 0,
  jonPrice: 0,
  paulPrice: 0,
  jonPays: (price: number) =>
    set((store) => ({
      fullPrice: store.fullPrice + price,
      jonPrice: store.jonPrice + price,
    })),
  paulPays: (price: number) =>
    set((store) => ({
      fullPrice: store.fullPrice + price,
      paulPrice: store.paulPrice + price,
    })),
}));

export default useMathsStore;
