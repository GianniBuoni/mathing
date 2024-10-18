import Button from "./Button";
import axios from "axios";
import useFormStore from "@/stores/useFormStore";
import { motion } from "framer-motion";
import type { Variants } from "framer-motion";

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
    <motion.div variants={variants} initial="initial" animate="animate">
      <Button color="secondary" hover="secondary" onClick={() => handleClick()}>
        Delete Item
      </Button>
    </motion.div>
  );
};

const variants: Variants = {
  initial: { opacity: 0 },
  animate: { opacity: 1, transition: { duration: 0.5 } },
};

export default DeleteButton;
