import useFormStore from "@/stores/useFormStore";
import { motion, AnimatePresence } from "framer-motion";
import type { Variants } from "framer-motion";

const FilledFormArray = () => {
  const { filledForms, modFilledForms } = useFormStore();
  const handleSplice = (i: number) => {
    let newArray = filledForms;
    newArray.splice(i, 1);
    modFilledForms(newArray);
  };
  return (
    <div className="grid grid-cols-10 gap-2">
      <AnimatePresence mode="popLayout">
        {filledForms.map((form, i) => (
          <motion.div
            className="rounded-box bg-neutral px-5 py-2 grid grid-cols-subgrid col-span-full items-center"
            key={`filledForm-${i}`}
            layoutId={`filledForm-${i}`}
            variants={variants}
            initial="initial"
            animate="animate"
            exit="exit"
          >
            <p className="col-span-3">
              {form.item.item} : ${form.item.price}
            </p>
            <p className="flex gap-3 col-span-2">Quantity: {form.quantity}</p>
            <p className="col-span-3">
              {form.payee === "half" && (
                <span>Both Pay: ${form.payeePrice.toFixed(2)}</span>
              )}
              {form.payee === "jon" && (
                <span>Jon Pays: ${form.payeePrice.toFixed(2)}</span>
              )}
              {form.payee === "paul" && (
                <span>Paul Pays: ${form.payeePrice.toFixed(2)}</span>
              )}
            </p>
            <button
              className="btn btn-circle btn-sm btn-secondary scale-90 hover:scale-95 text-sm pb-1 col-end-11 justify-self-end"
              onClick={() => handleSplice(i)}
            >
              x
            </button>
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
};

const variants: Variants = {
  initial: { opacity: 0, y: 100 },
  animate: { opacity: 1, y: 0 },
  exit: { opacity: 0, y: 100 },
};

export default FilledFormArray;
