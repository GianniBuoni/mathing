import { useForm } from "react-hook-form";

import Button from "./Button";
import useFormStore from "@/stores/useFormStore";
import type { Payee } from "@/stores/useFormStore";

type PayObject = {
  quantity: number;
  payee: Payee;
};

const FormCard = () => {
  const { register, handleSubmit } = useForm<PayObject>();
  const { isFormActive, reset, activeFormItem } = useFormStore();

  const onSubmit = (data: PayObject) => {
    console.log({
      finalPrice: (data.quantity * activeFormItem.price).toFixed(2),
      payee: data.payee,
    });
    reset();
  };
  return (
    isFormActive && (
      <form
        className="rounded-box bg-base-300 p-5 flex gap-10 my-5 w-fit items-center"
        onSubmit={handleSubmit(onSubmit)}
      >
        <p>
          {activeFormItem.item}: ${activeFormItem.price}
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
    )
  );
};

export default FormCard;
