import { useForm } from "react-hook-form";

import Button from "./Button";
import useFormStore from "@/stores/useFormStore";
import type { FilledForm, Payee } from "@/stores/useFormStore";

type PayObject = {
  quantity: number;
  payee: Payee;
};

const FormCard = () => {
  // to do: break up what you can to helper functions
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<PayObject>();
  const { isFormActive, filledForms, reset, activeFormItem, modFilledForms } =
    useFormStore();

  const onSubmit = (data: PayObject) => {
    // to do: add zod validation
    const finalPrice = activeFormItem.price * data.quantity;
    console.log(finalPrice);
    const payeePrice = data.payee === "half" ? finalPrice / 2 : finalPrice;
    console.log(payeePrice);

    const newArrayData: FilledForm = {
      item: activeFormItem,
      quantity: data.quantity,
      payee: data.payee,
      payeePrice: payeePrice,
    };

    const newArray = [...filledForms, newArrayData];
    modFilledForms(newArray);
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
        {/* to do: add error handling and validation for payee field
        choose should not be a valid input if no selection is made */}
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
        <div className="flex gap-2">
          <Button type="submit" color="primary">
            Submit
          </Button>
          <Button onClick={() => reset()} color="secondary">
            Cancel
          </Button>
        </div>
      </form>
    )
  );
};

export default FormCard;
