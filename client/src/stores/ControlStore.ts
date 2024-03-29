import { create } from "zustand";
import { Controls, Modus } from "../models/ControlModels";
import { get, post } from "../service/apiService";

export interface ControlStore extends Controls {
  getControlsAsync: () => Promise<void>;
  setModus: (modus: Modus) => Promise<void>;
}

export const useControlStore = create<ControlStore>((set) => ({
  modus: Modus.Off,
  lat: 0,
  lon: 0,
  status: false,
  getControlsAsync: async () => {
    get("controls").then((data) => set(data));
  },
  setModus: async (modus) => {
    post("modus", { modus: modus }).then((data) => set(data));
  },
}));

get("controls").then((data) => useControlStore.setState(data));
