import { create } from "zustand";
import { Setting } from "../models/SettingsModel";
import { get, post } from "../service/apiService";

export interface SettingsStore extends Setting {
  getSettingsAsync: () => Promise<void>;
  setSettings: (settings: Setting) => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  lat: 0,
  lon: 0,
  house_rotation: 0,
  roof_inclination: 0,
  start_value: 0,
  end_value: 0,
  submitted: true,
  pin: 0,
  default_high: true,
  getSettingsAsync: async () => {
    get("settings").then((data) => data && set({ submitted: true, ...data }));
  },
  setSettings: async (settings) => {
    post("settings", settings).then(
      (data) => data && set({ submitted: true, ...data })
    );
  },
}));

get("settings").then(
  (data) => data && useSettingsStore.setState({ submitted: true, ...data })
);
