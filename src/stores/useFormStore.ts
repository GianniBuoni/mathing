import type { Item } from "@/db/types";
import { z } from "astro:content";
import { create } from "zustand";

export type FormArrayState = {
  isFormActive: boolean;
  isAddActive: boolean;
  isEditActive: boolean;
  activeFormItem: Item;
  filledForms: FilledForm[];
  modFilledForms: (i: FilledForm[]) => void;
  activate: () => void;
  activateAdd: () => void;
  activateEdit: () => void;
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
  isAddActive: false,
  isEditActive: false,
  activeFormItem: {} as Item,
  filledForms: [],
  modFilledForms: (i: FilledForm[]) =>
    set(() => ({
      filledForms: i,
    })),
  activate: () => set(() => ({ isFormActive: true })),
  activateAdd: () => set(() => ({ isAddActive: true })),
  activateEdit: () => set(() => ({ isEditActive: true })),
  passItemProp: (i: Item) =>
    set(() => ({
      activeFormItem: i,
    })),
  reset: () =>
    set(() => ({
      isFormActive: false,
      isAddActive: false,
      isEditActive: false,
      activeFormItem: {} as Item,
    })),
}));

export default useFormStore;
