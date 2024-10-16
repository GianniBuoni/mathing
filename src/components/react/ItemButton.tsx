import useFormStore from "@/stores/useFormStore";
import Button from "./Button";
import type { Item } from "@/db/schema";

interface Props {
  item: Item;
}

const ItemButton = ({ item }: Props) => {
  const { isFormActive, activate, passItemProp } = useFormStore();
  const handleClick = () => {
    activate();
    passItemProp(item);
  };
  return (
    <Button
      color="primary"
      disabled={isFormActive}
      onClick={() => handleClick()}
    >
      {item.item}
    </Button>
  );
};

export default ItemButton;
