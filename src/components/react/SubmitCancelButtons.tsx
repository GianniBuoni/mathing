import Button from "./Button";
import useFormStore from "@/stores/useFormStore";

const SubmitCancelButtons = () => {
  const { reset } = useFormStore();
  return (
    <div className="flex gap-2">
      <Button type="submit" color="primary">
        Submit
      </Button>
      <Button onClick={() => reset()} color="secondary">
        Cancel
      </Button>
    </div>
  );
};

export default SubmitCancelButtons;
