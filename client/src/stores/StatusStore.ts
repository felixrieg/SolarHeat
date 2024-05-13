import { create } from "zustand";
import { get } from "../service/apiService";

export interface StatusStore {
  status: boolean;
  getStatus: () => Promise<void>;
}

export const useStatusStore = create<StatusStore>((set, getState) => ({
  status: false,
  getStatus: async () => {
    get("status").then((data) => set(data));
  },
}));

get("status").then((data) => useStatusStore.setState(data));
