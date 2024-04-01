import { create } from "zustand";
import { Controls, Modus } from "../models/ControlModels";
import { get, post } from "../service/apiService";

export interface ControlStore extends Controls {
  getControlsAsync: () => Promise<void>;
  setModus: (modus: Modus) => Promise<void>;
  setWeather: (weather: number) => Promise<void>;
}

export const useControlStore = create<ControlStore>((set) => ({
  modus: Modus.Off,
  weather: 0.5,
  getControlsAsync: async () => {
    get("controls").then((data) => set(data));
  },
  setModus: async (modus) => {
    post("controls/modus", { modus: modus }).then((data) => set(data));
  },
  setWeather: async (weather) => {
    post("controls/weather", { weather: weather }).then((data) => set(data));
  },
}));

get("controls").then((data) => useControlStore.setState(data));
