import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { AnimatePresence, motion } from "framer-motion";
import type { Variants } from "framer-motion";

import useFormStore, { PayObjectSchema } from "@/stores/useFormStore";
import type { FilledForm, PayObject } from "@/stores/useFormStore";
import SubmitCancelButtons from "./SubmitCancelButtons";

const FormCard = () => {
  const { register, handleSubmit } = useForm<PayObject>({
    resolver: zodResolver(PayObjectSchema),
  });

  const { isFormActive, reset, activeFormItem, filledForms, modFilledForms } =
    useFormStore();

  const onSubmit = (data: PayObject) => {
    const finalPrice = activeFormItem.price * data.quantity;
    const payeePrice = data.payee === "half" ? finalPrice / 2 : finalPrice;

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
    <AnimatePresence mode="popLayout">
      {isFormActive && (
        <motion.form
          className="rounded-box bg-base-300 p-5 flex gap-10 w-fit items-center"
          onSubmit={handleSubmit(onSubmit)}
          key="formCard"
          variants={variants}
          initial="initial"
          animate="animate"
          exit="exit"
        >
          <p>
            {activeFormItem.item}: ${activeFormItem.price.toFixed(2)}
          </p>
          <label className="flex gap-3 items-center">
            Quantity:
            <input
              type="number"
              size={1}
              defaultValue={1}
              className="rounded p-1"
              {...register("quantity")}
            />
          </label>
          <select
            id="payee"
            defaultValue="choose"
            className="rounded p-2"
            {...register("payee")}
          >
            <option value="choose" disabled>
              Who Pays?
            </option>
            <option value="jon">Jon</option>
            <option value="paul">Paul</option>
            <option value="half">Halvsies</option>
          </select>
          <SubmitCancelButtons />
        </motion.form>
      )}
    </AnimatePresence>
  );
};

const variants: Variants = {
  initial: { opacity: 0, y: -300 },
  animate: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: -300 },
};
export default FormCard;
