import { useEffect, useState } from "react";
import { useStatusStore } from "../stores/StatusStore";
import "./styles/Status.scss";

const Status = () => {
  const status = useStatusStore((state) => state);

  const [running, setRunning] = useState(status.status);

  useEffect(() => {
    setRunning(status.status);
  }, [status]);

  useEffect(() => {
    var interval = setInterval(() => {
      status.getStatus();
    }, 5000);
    return () => {
      clearInterval(interval);
    };
    // eslint-disable-next-line
  }, []);

  return (
    <div className="status">
      Status:
      <div className={running ? "running" : "off"}>
        {running ? "running" : "off"}
      </div>
    </div>
  );
};

export default Status;
