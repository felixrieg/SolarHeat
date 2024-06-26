import { useEffect, useState } from "react";
import { useSettingsStore } from "../stores/SettingsStore";
import "./styles/Settings.scss";
import SettingsLabel from "../components/SettingsLabel";
import { Setting } from "../models/SettingsModel";

type Props = {
  close?: () => void;
};

const Settings = ({ close }: Props) => {
  const settings = useSettingsStore((state) => state);
  const [submitted, setSubmitted] = useState(true);
  const [lat, setLat] = useState(settings.lat);
  const [lon, setLon] = useState(settings.lon);
  const [house_rotation, setHouse_rotation] = useState(settings.house_rotation);
  const [roof_inclination, setRoof_inclination] = useState(
    settings.roof_inclination
  );
  const [start_value, setStart_value] = useState(settings.start_value);
  const [end_value, setEnd_value] = useState(settings.end_value);
  const [pin, setPin] = useState(settings.pin);
  const [default_high, setDefault_high] = useState(!!settings.default_high);

  const handleSubmit = (event: any) => {
    const new_settings: Setting = {
      lat,
      lon,
      house_rotation,
      roof_inclination,
      start_value,
      end_value,
      pin,
      default_high,
    };
    settings.setSettings(new_settings);
    event.preventDefault();
    close && close();
  };

  const change_value = (
    setter: (number: number) => void
  ): ((number: number) => void) => {
    return (x: number) => {
      setter(x);
      setSubmitted(false);
    };
  };

  useEffect(() => {
    setSubmitted(true);
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
            setValue={change_value(setRoof_inclination)}
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
          <label className="switch">
            <div className="settings-label">Pin default high:</div>

            <input
              type="checkbox"
              name={"default_high"}
              checked={default_high}
              onChange={(v) => {
                setDefault_high(v.target.checked);
                setSubmitted(false);
              }}
            />
            <span className="slider round"></span>
          </label>
          <input type="submit" value="Submit" disabled={submitted} />
        </form>
      </div>
    </div>
  );
};
export default Settings;
