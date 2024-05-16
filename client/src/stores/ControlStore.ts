import { create } from "zustand";
import { Controls, Modus } from "../models/ControlModels";
import { get, post } from "../service/apiService";

export interface ControlStore extends Controls {
  getControlsAsync: () => Promise<void>;
  setModus: (modus: Modus) => Promise<void>;
}

export const useControlStore = create<ControlStore>((set, getState) => ({
  modus: Modus.Off,
  weather: 0.5,
  getControlsAsync: async () => {
    get("controls").then((data) => data && set(data));
  },
  setModus: async (modus) => {
    console.log("set modus", { ...getState(), modus: modus });
    post("controls", { ...getState(), modus: modus }).then(
      (data) => data && set(data)
    );
  },
}));

get("controls").then((data) => data && useControlStore.setState(data));
