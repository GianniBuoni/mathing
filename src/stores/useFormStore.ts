import type { Item } from "@/db/schema";
import { create } from "zustand";

export type FormArrayState = {
  isFormActive: boolean;
  activeFormItem: Item;
  filledForms: FilledForm[];
  modFilledForms: (i: FilledForm[]) => void;
  activate: () => void;
  passItemProp: (i: Item) => void;
  reset: () => void;
};

export type FilledForm = {
  item: Item;
  quantity: number;
  payee: Payee;
};

export type Payee = "jon" | "paul" | "half";

const useFormStore = create<FormArrayState>((set) => ({
  isFormActive: false,
  activeFormItem: {} as Item,
  filledForms: [],
  modFilledForms: (i: FilledForm[]) =>
    set(() => ({
      filledForms: i,
    })),
  activate: () => set(() => ({ isFormActive: true })),
  passItemProp: (i: Item) =>
    set(() => ({
      activeFormItem: i,
    })),
  reset: () =>
    set(() => ({
      isFormActive: false,
    })),
}));

export default useFormStore;
