import { useEffect, useState } from "react";
import { useControlStore } from "../stores/ControlStore";
import { Modus } from "../models/ControlModels";
import "./styles/Controls.scss";

const Controls = () => {
  const controls = useControlStore((state) => state);

  const possibleModi = [Modus.Off, Modus.Single, Modus.Continuous, Modus.On];
  const [modus, setModus] = useState(
    possibleModi.findIndex((x) => x === controls.modus)
  );

  useEffect(() => {
    setModus(possibleModi.findIndex((x) => x === controls.modus));
    // eslint-disable-next-line
  }, [controls]);

  useEffect(() => {
    var interval = setInterval(() => {
      controls.getControlsAsync();
    }, 5000);
    return () => {
      clearInterval(interval);
    };
    // eslint-disable-next-line
  }, []);

  const onChange = (v: any) => {
    console.log(
      "new value",
      v.target.value,
      possibleModi[+v.target.value],
      modus
    );
    controls?.setModus &&
      controls?.setModus(possibleModi[+v.target.value] ?? Modus.Off);
    // setModus(v);
  };

  return (
    <div className="Controls">
      <div>Modus:</div>
      <select
        name="modus"
        className="invisibleSelect"
        onChange={onChange}
        value={modus}
      >
        {possibleModi.map((m, i) => (
          <option key={i} value={i}>
            {m}
          </option>
        ))}
      </select>
    </div>
  );
};

export default Controls;
