import { useEffect, useState } from "react";
import { useControlStore } from "../stores/ControlStore";
import { Modus } from "../models/ControlModels";
import CustomSlider from "../components/CustomSlider";
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

  return (
    <div className="Controls">
      <CustomSlider
        left={Modus.Off}
        right={Modus.On}
        label="Modus: "
        showCurrent
        values={possibleModi}
        value={modus}
        setValue={(v) => {
          console.log("new value", v, controls, controls.setModus);
          controls?.setModus &&
            controls?.setModus(possibleModi[v] ?? Modus.Off);
          // setModus(v);
        }}
      />
    </div>
  );
};

export default Controls;
