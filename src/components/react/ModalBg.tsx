import useFormStore from "@/stores/useFormStore";
import EditAddForm from "@/components/react/EditAddForm";
import DeleteButton from "@/components/react/DeleteButton";

const ModalBg = () => {
  const { activeFormItem, isAddActive, isEditActive } = useFormStore();
  return (
    (isAddActive || isEditActive) && (
      <div className="fixed top-0 left-0 w-screen h-screen bg-base-300 bg-opacity-95 flex flex-col justify-center items-center z-30">
        <EditAddForm
          item={{
            item: activeFormItem.item,
            price: activeFormItem.price,
          }}
        />
        {isEditActive && <DeleteButton />}
      </div>
    )
  );
};

export default ModalBg;
