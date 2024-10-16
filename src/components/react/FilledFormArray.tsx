import useFormStore from "@/stores/useFormStore";

const FilledFormArray = () => {
  const { filledForms, modFilledForms } = useFormStore();
  const handleSplice = (i: number) => {
    let newArray = filledForms;
    newArray.splice(i, 1);
    modFilledForms(newArray);
  };
  return filledForms.map((form, i) => (
    <div className="rounded-box bg-neutral p-5 flex gap-10 my-2 w-fit items-center">
      <p>
        {form.item.item} : ${form.item.price}
      </p>
      <p className="flex gap-3">Quantity: {form.quantity}</p>
      {form.payee === "half" && <p>Both Pay: {form.payeePrice}</p>}
      {form.payee === "jon" && <p>Jon Pays: {form.payeePrice}</p>}
      {form.payee === "paul" && <p>Paul Pays: {form.payeePrice}</p>}
      <button
        className="btn btn-circle btn-sm btn-secondary scale-90 hover:scale-95 text-sm pb-1"
        onClick={() => handleSplice(i)}
      >
        x
      </button>
    </div>
  ));
};

export default FilledFormArray;
