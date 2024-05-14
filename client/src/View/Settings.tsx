import { useEffect, useState } from "react";
import { useSettingsStore } from "../stores/SettingsStore";
import "./styles/Settings.scss";
import SettingsLabel from "../components/SettingsLabel";
import { Setting } from "../models/SettingsModel";

const Settings = () => {
  const settings = useSettingsStore((state) => state);
  const [lat, setLat] = useState(settings.lat);
  const [lon, setLon] = useState(settings.lon);
  const [house_rotation, setHouse_rotation] = useState(settings.house_rotation);
  const [roof_inclination, setRoof_inclination] = useState(
    settings.roof_inclination
  );
  const [start_value, setStart_value] = useState(settings.start_value);
  const [end_value, setEnd_value] = useState(settings.end_value);
  const [pin, setPin] = useState(settings.pin);

  const handleSubmit = (event: any) => {
    console.log(event);

    const new_settings: Setting = {
      lat,
      lon,
      house_rotation,
      roof_inclination,
      start_value,
      end_value,
      pin,
    };

    console.log("new Settings", new_settings);
    settings.setSettings(new_settings);

    event.preventDefault();
  };

  const change_value = (
    setter: (number: number) => void
  ): ((number: number) => void) => {
    return (x: number) => {
      setter(x);
      if (settings.submitted) {
        settings.setSubmitted(false);
      }
    };
  };

  useEffect(() => {
    console.log(settings);
    setLat(settings.lat);
    setLon(settings.lon);
    setHouse_rotation(settings.house_rotation);
    setRoof_inclination(settings.roof_inclination);
    setStart_value(settings.start_value);
    setEnd_value(settings.end_value);
    setPin(settings.pin);
  }, [settings]);

  return (
    <div className="settings">
      <div className="headline">Settings</div>
      <div>
        <form className="settingsForm" onSubmit={handleSubmit}>
          <SettingsLabel
            value={lat}
            setValue={change_value(setLat)}
            name="lat"
            label="Lat"
          />
          <SettingsLabel
            value={lon}
            setValue={change_value(setLon)}
            name="lon"
            label="Lon"
          />
          <div className="divider" />
          <SettingsLabel
            value={house_rotation}
            setValue={change_value(setHouse_rotation)}
            name="house_rotation"
            label="House rotation"
            min={-90}
            max={90}
            range
          />
          <SettingsLabel
            value={roof_inclination}
            setValue={setRoof_inclination}
            name="roof_inclination"
            label="Roof inclination"
            min={-90}
            max={90}
            range
          />
          <div className="divider" />
          <SettingsLabel
            value={start_value}
            setValue={change_value(setStart_value)}
            name="start_value"
            label="Start value"
            min={-90}
            max={90}
            range
          />
          <SettingsLabel
            value={end_value}
            setValue={change_value(setEnd_value)}
            name="end_value"
            label="End value"
            min={-90}
            max={90}
            range
          />
          <div className="divider" />
          <SettingsLabel
            value={pin}
            setValue={change_value(setPin)}
            name="pin"
            label="(RPI) Pin"
          />
          <input type="submit" value="Submit" disabled={settings.submitted} />
        </form>
      </div>
    </div>
  );
};
export default Settings;
