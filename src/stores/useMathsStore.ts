import { create } from "zustand";
export type MathsState = {
  fullPrice: number[];
  jonPrice: number[];
  paulPrice: number[];
  modPrices: (mod: ModifiedArrays) => void;
};

type ModifiedArrays = {
  modFull: [];
  modJon: [];
  modPaul: [];
};

const useMathsStore = create<MathsState>((set) => ({
  fullPrice: [],
  jonPrice: [],
  paulPrice: [],
  modPrices: (mod: ModifiedArrays) =>
    set(() => ({
      fullPrice: mod.modFull,
      jonPrice: mod.modJon,
      paulPrice: mod.modPaul,
    })),
}));

export default useMathsStore;
