import { useEffect, useState } from "react";
import axios from "axios";

// lib
import useFormStore from "@/stores/useFormStore";

//ui
import ItemButton from "./ItemButton";

const ItemArray = () => {
  const [allItems, setAllItems] = useState([]);
  const { refetch } = useFormStore();

  useEffect(() => {
    const getAllItems = async () => {
      await axios
        .get(`/api/getAllItems.json`)
        .then((res) => setAllItems(res.data))
        .catch((e) => console.log(e));
    };
    getAllItems();
  }, [refetch]);
  return (
    <ul className="flex flex-wrap gap-3">
      {allItems.map((item, i) => (
        <li key={`item-array-${i}`}>
          <ItemButton item={item} />
        </li>
      ))}
    </ul>
  );
};

export default ItemArray;
