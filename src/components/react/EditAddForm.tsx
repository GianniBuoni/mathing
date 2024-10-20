// modules
import { AnimatePresence, motion } from "framer-motion";
import axios from "axios";
import { useForm } from "react-hook-form";
import type { Variants } from "framer-motion";
import { z } from "zod";

// lib
import useFormStore from "@/stores/useFormStore";
import { zodResolver } from "@hookform/resolvers/zod";

// ui
import SubmitCancelButtons from "@/components/react/SubmitCancelButtons";

const AddEditSchema = z.object({
  item: z.string(),
  price: z.coerce.number(),
});

type AddEditItem = z.infer<typeof AddEditSchema>;

// for some reason, react-hook-form is having reading from store
// passing the activeFormItem as a prop works for the inputs
interface Props {
  item?: AddEditItem;
}

const EditAddForm = ({ item }: Props) => {
  const { activeFormItem, isAddActive, reset, doRefetch } = useFormStore();
  const { register, handleSubmit, watch } = useForm<AddEditItem>({
    resolver: zodResolver(AddEditSchema),
  });

  watch();

  const onSubmit = async (data: AddEditItem) => {
    try {
      // validate data
      const validation = AddEditSchema.safeParse(data);
      if (!validation) throw new Error("Data does not match Insert Schema");

      if (isAddActive) {
        // add item
        await axios
          .post(`/api/addItem.json`, data)
          .then(() => doRefetch())
          .catch((error) => console.log(error))
          .finally(() => reset());
      } else {
        // edit item
        await axios
          .patch(`/api/${activeFormItem.id}.json`, data)
          .then(() => doRefetch())
          .catch((error) => console.log(error))
          .finally(() => reset());
      }
    } catch (e) {
      console.log(e);
    }
  };
  return (
    <AnimatePresence>
      <motion.form
        variants={variants}
        initial={`initial`}
        animate={`animate`}
        layoutId="AddEditForm"
        className="rounded-box bg-neutral shadow-2xl p-5 flex gap-5 my-5 w-fit items-center"
        onSubmit={handleSubmit(onSubmit)}
      >
        <input
          placeholder="Item name"
          className={inputClasses}
          type="text"
          defaultValue={item?.item}
          {...register("item", { required: true })}
        />
        <input
          placeholder="Item Price"
          className={inputClasses}
          defaultValue={item?.price}
          type="text"
          {...register("price", { required: true })}
        />
        <SubmitCancelButtons />
      </motion.form>
    </AnimatePresence>
  );
};

const inputClasses = "rounded p-1";

const variants: Variants = {
  initial: { opacity: 0, y: 300 },
  animate: { opacity: 1, y: 0 },
};

export default EditAddForm;
