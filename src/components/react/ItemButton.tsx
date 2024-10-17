import useFormStore from "@/stores/useFormStore";
import Button from "./Button";
import type { Item } from "@/db/types";

interface Props {
  item: Item;
}

const ItemButton = ({ item }: Props) => {
  const { isFormActive, activate, passItemProp, activeFormItem } =
    useFormStore();
  const handleClick = () => {
    activate();
    passItemProp(item);
  };
  return (
    <Button
      color="primary"
      disabled={isFormActive && item.id != activeFormItem.id}
      onClick={() => handleClick()}
    >
      {item.item}
    </Button>
  );
};

export default ItemButton;
