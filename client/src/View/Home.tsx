import { useState } from "react";
import Controls from "./Controls";
import SlideIn from "../components/SlideIn";
import Settings from "./Settings";
import "./styles/Home.scss";
import SettingsIcon from "@mui/icons-material/Settings";

const Home = () => {
  const [showSettings, setShowSettings] = useState(false);

  return (
    <div className="Home">
      <div className="Content">
        <div className="headline">SolarHeat</div>
        <Controls />

        <div
          className="clickable settingsbutton"
          onClick={() => setShowSettings(true)}
        >
          <SettingsIcon />
        </div>
        <SlideIn
          show={showSettings}
          onClose={() => {
            setShowSettings(false);
          }}
        >
          <Settings />
        </SlideIn>
      </div>
    </div>
  );
};

export default Home;
