import { useState } from "react";
import Controls from "./Controls";
import SlideIn from "../components/SlideIn";
import Settings from "./Settings";
import "./styles/Home.scss";
import SettingsIcon from "@mui/icons-material/Settings";
import Status from "./Status";
import { useSettingsStore } from "../stores/SettingsStore";
import WidgetContainer from "../components/WidgetContainer";

const Home = () => {
  const getSettingsAsync = useSettingsStore((state) => state.getSettingsAsync);
  const [showSettings, setShowSettings] = useState(false);

  return (
    <div className="Home">
      <div className="header">
        <div className="headline">SolarHeat</div>
      </div>
      <div className="Content">
        <div className="homeGrid">
          <WidgetContainer>
            <Status />
          </WidgetContainer>
          <WidgetContainer>
            <Controls />
          </WidgetContainer>
        </div>

        <SlideIn
          show={showSettings}
          onClose={() => {
            setShowSettings(false);
            getSettingsAsync();
          }}
        >
          <Settings
            close={() => {
              setShowSettings(false);
              getSettingsAsync();
            }}
          />
        </SlideIn>
      </div>
      <div className="footer">
        <div
          className="clickable"
          onClick={() => {
            getSettingsAsync();
            setShowSettings(true);
          }}
        >
          <SettingsIcon />
        </div>
      </div>
    </div>
  );
};

export default Home;
