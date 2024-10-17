import useFormStore from "@/stores/useFormStore";
import EditAddForm from "@/components/react/EditAddForm";

const ModalBg = () => {
  const { activeFormItem, isAddActive, isEditActive, isDeleteActive } =
    useFormStore();
  return (
    (isAddActive || isEditActive || isDeleteActive) && (
      <div className="fixed top-0 left-0 w-screen h-screen bg-base-300 bg-opacity-95 flex justify-center items-center">
        <EditAddForm
          item={{
            item: activeFormItem.item,
            price: activeFormItem.price,
          }}
        />
      </div>
    )
  );
};

export default ModalBg;
