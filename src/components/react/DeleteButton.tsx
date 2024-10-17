import Button from "./Button";
import axios from "axios";
import useFormStore from "@/stores/useFormStore";

const DeleteButton = () => {
  const { reset, activeFormItem } = useFormStore();
  const handleClick = () => {
    if (
      confirm(
        `Are you sure you want to delete this item?\nThis process is not reversible.`,
      )
    )
      axios
        .delete(`/api/${activeFormItem.id}.json`)
        .catch((e) => console.log(e))
        .finally(() => reset());
  };

  return (
    <Button color="secondary" hover="secondary" onClick={() => handleClick()}>
      Delete Item
    </Button>
  );
};

export default DeleteButton;
