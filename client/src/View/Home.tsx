import { useEffect, useState } from "react";
import Controls from "./Controls";
import SlideIn from "../components/SlideIn";
import Settings from "./Settings";
import "./styles/Home.scss";
import SettingsIcon from "@mui/icons-material/Settings";
import Status from "./Status";
import { useSettingsStore } from "../stores/SettingsStore";

const Home = () => {
  const getSettingsAsync = useSettingsStore((state) => state.getSettingsAsync);
  const [showSettings, setShowSettings] = useState(false);

  return (
    <div className="Home">
      <div className="Content">
        <div className="headline">SolarHeat</div>
        <Status />
        <div className="vspace"></div>
        <div className="divider" />
        <Controls />

        <div
          className="clickable settingsbutton"
          onClick={() => {
            getSettingsAsync();
            setShowSettings(true);
          }}
        >
          <SettingsIcon />
        </div>
        <SlideIn
          show={showSettings}
          onClose={() => {
            setShowSettings(false);
            getSettingsAsync();
          }}
        >
          <Settings />
        </SlideIn>
      </div>
    </div>
  );
};

export default Home;
