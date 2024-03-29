export type Controls = {
  modus: Modus;
  lat: number;
  lon: number;
  status: boolean;
};

export enum Modus {
  "Off" = "Off",
  "Single" = "Single",
  "Continuous" = "Continuous",
}
