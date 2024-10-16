import useFormStore from "@/stores/useFormStore";

const Counter = () => {
  const { filledForms } = useFormStore();
  const jonOnly = filledForms.filter((item) => item.payee === "jon");
  const paulOnly = filledForms.filter((item) => item.payee === "paul");
  const halved = filledForms.filter((item) => item.payee === "half");
  const allJon = [...jonOnly, ...halved].reduce(
    (acc, item) => acc + item.payeePrice,
    0,
  );
  const allPaul = [...paulOnly, ...halved].reduce(
    (acc, item) => acc + item.payeePrice,
    0,
  );
  return (
    <div className="flex gap-5 font-display tracking-wider text-white">
      <p>JON ${allJon.toFixed(2)}</p>
      <p>PAUL ${allPaul.toFixed(2)}</p>
      <p>ALL ${(allPaul + allJon).toFixed(2)}</p>
    </div>
  );
};

export default Counter;
