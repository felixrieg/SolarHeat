import { create } from "zustand";
import { Setting } from "../models/SettingsModel";
import { get, post } from "../service/apiService";

export interface SettingsStore extends Setting {
  sumbitted: boolean;
  getSettingsAsync: () => Promise<void>;
  setPosition: () => void;
  setSettings: (settings: Setting) => Promise<void>;
  setSumbitted: (new_val: boolean) => void;
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  lat: 0,
  lon: 0,
  house_rotation: 0,
  roof_inclination: 0,
  start_value: 0,
  end_value: 0,
  sumbitted: true,
  getSettingsAsync: async () => {
    get("settings").then((data) => set({ sumbitted: true, ...data }));
  },
  setPosition: () => {},
  setSettings: async (settings) => {
    post("settings", settings).then((data) =>
      set({ sumbitted: true, ...data })
    );
  },
  setSumbitted: (new_val) => {
    set({ sumbitted: new_val });
  },
}));

get("settings").then((data) =>
  useSettingsStore.setState({ sumbitted: true, ...data })
);
