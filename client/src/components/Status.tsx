import { useControlStore } from "../stores/ControlStore";

const Status = () => {
  const status = useControlStore((state) => state.status);
  return <div style={{ color: status ? "green" : "red" }}>status</div>;
};

export default Status;
