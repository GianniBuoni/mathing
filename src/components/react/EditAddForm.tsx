// modules
import { z } from "astro:content";
import { useForm } from "react-hook-form";
import axios from "axios";

// lib
import useFormStore from "@/stores/useFormStore";
import { zodResolver } from "@hookform/resolvers/zod";

// ui
import SubmitCancelButtons from "./SubmitCancelButtons";

const AddEditSchema = z.object({
  item: z.string(),
  price: z.coerce.number(),
});
type AddEditItem = z.infer<typeof AddEditSchema>;

const EditAddForm = () => {
  const { activeFormItem, isAddActive, isEditActive, reset } = useFormStore();
  const { register, handleSubmit } = useForm<AddEditItem>({
    resolver: zodResolver(AddEditSchema),
  });

  const onSubmit = async (data: AddEditItem) => {
    try {
      // validate data
      const validation = AddEditSchema.safeParse(data);
      if (!validation) throw new Error("Data does not match Insert Schema");

      if (isAddActive) {
        // add item
        await axios
          .post(`${import.meta.env.BASE_URL}/api/addItem.json`, { data })
          .then((res) => console.log(res))
          .catch((error) => console.log(error))
          .finally(() => reset());
      } else {
        // edit item
        await axios
          .patch(`${import.meta.env.BASE_URL}/api/${activeFormItem.id}`, {
            data,
          })
          .then((res) => console.log(res))
          .catch((error) => console.log(error))
          .finally(() => reset());
      }
    } catch (e) {
      console.log(e);
    }
  };
  return (
    <form
      className="rounded-box bg-base-300 p-5 flex gap-10 my-5 w-fit items-center"
      onSubmit={handleSubmit(onSubmit)}
    >
      <label>
        Item Name:
        <input
          defaultValue={isEditActive ? activeFormItem.item : ""}
          type="text"
          {...register("item")}
        />
      </label>
      <label>
        Item Price:
        <input
          defaultValue={isEditActive ? activeFormItem.price : ""}
          type="text"
          {...register("price")}
        />
      </label>
      <SubmitCancelButtons />
    </form>
  );
};

export default EditAddForm;
