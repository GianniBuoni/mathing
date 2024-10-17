import React from "react";
import useFormStore from "@/stores/useFormStore";
import Button from "./Button";

type Button = typeof Button;
interface Props extends React.ComponentProps<Button> {}

const EditAddItem = ({ ...rest }: Props) => {
  const { isFormActive, reset, activateEdit, activateAdd } = useFormStore();
  if (!isFormActive)
    return (
      <Button hover="secondary" onClick={() => activateAdd()} {...rest}>
        Add Item
      </Button>
    );
  return (
    <div className="flex gap-1">
      <Button hover="secondary" onClick={() => activateEdit()} {...rest}>
        Edit Item
      </Button>
      <Button hover="secondary" onClick={() => reset()}>
        Cancel
      </Button>
    </div>
  );
};

export default EditAddItem;
