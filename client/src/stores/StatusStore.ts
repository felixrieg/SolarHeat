import { create } from "zustand";
import { get } from "../service/apiService";

export interface StatusStore {
  status: boolean;
  getStatus: () => Promise<void>;
}

const statusChanged = (new_status: StatusStore, status: StatusStore) => {
  return new_status.status !== status.status;
};

export const useStatusStore = create<StatusStore>((set, getState) => ({
  status: false,
  getStatus: async () => {
    get("status").then(
      (data) => data && statusChanged(data, getState()) && set(data)
    );
  },
}));

get("status").then((data) => data && useStatusStore.setState(data));
