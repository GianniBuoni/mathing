import type { Item } from "@/db/schema";
import { z } from "astro:content";
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
  payeePrice: number;
};

export type Payee = "jon" | "paul" | "half";

export const PayObjectSchema = z.object({
  quantity: z.coerce.number().min(1),
  payee: z.enum(["jon", "paul", "half"]),
});

export type PayObject = z.infer<typeof PayObjectSchema>;

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
