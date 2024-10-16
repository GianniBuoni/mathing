import React from "react";
import { useForm } from "react-hook-form";

import type { Item } from "@/db/schema";
import Button from "./Button";
import useMathsStore from "@/stores/useMathsStore";

interface Props {
  item: Item;
}

type PayObject = {
  quantity: number;
  payee: "jon" | "paul" | "half";
};

const FormCard = ({ item }: Props) => {
  const { register, handleSubmit } = useForm<PayObject>();

  const onSubmit = (data: PayObject) =>
    console.log({
      finalPrice: (data.quantity * item.price).toFixed(2),
      payee: data.payee,
    });

  return (
    <form
      className="rounded-box bg-base-300 p-5 flex gap-10 my-5 w-fit items-center"
      onSubmit={handleSubmit(onSubmit)}
    >
      <p>
        {item.item}: ${item.price}
      </p>
      <label className="flex gap-3">
        Quantity:
        <input
          type="number"
          size={1}
          defaultValue={1}
          {...register("quantity", { min: 1 })}
        />
      </label>
      <select
        id="payee"
        defaultValue="choose"
        {...register("payee", { required: true })}
      >
        <option value="choose" disabled>
          Who Pays?
        </option>
        <option value="jon">Jon</option>
        <option value="paul">Paul</option>
        <option value="half">Halvsies</option>
      </select>
      <Button type="submit" color="primary">
        Submit
      </Button>
    </form>
  );
};

export default FormCard;
