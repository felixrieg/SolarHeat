export type Controls = {
  modus: Modus;
  weather: number;
};

export enum Modus {
  "Off" = "Off",
  "Single" = "Single",
  "Continuous" = "Continuous",
  "On" = "On",
}
