import "./App.css";
import { Modus } from "./models/ControlModels";
import { FormControl, InputLabel, MenuItem, Select } from "@mui/material";
import { useControlStore } from "./stores/ControlStore";

function App() {
  const controls = useControlStore((state) => state);

  return (
    <div className="App-header">
      <FormControl>
        <InputLabel id="modus-select">Mode</InputLabel>

        <Select
          labelId="modus-select"
          value={controls.modus}
          label="Modus"
          onChange={(x) => controls.setModus(x.target.value as Modus)}
        >
          <MenuItem value={Modus.Off}>Off</MenuItem>
          <MenuItem value={Modus.Single}>Single</MenuItem>
          <MenuItem value={Modus.Continuous}>Continuous</MenuItem>
        </Select>
      </FormControl>
      <div>weather: {controls.weather}</div>
    </div>
  );
}

export default App;
