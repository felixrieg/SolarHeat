type Props = {
  name: string;
  label: string;
  min?: number;
  max?: number;
  value: number;
  setValue: (x: number) => void;
  range?: boolean;
};

const SettingsLabel = (props: Props) => {
  return (
    <label>
      <div className="settings-label">{props.label}</div>
      {!!props.range && (
        <input
          type="range"
          name={props.name}
          value={props.value}
          min={props.min}
          max={props.max}
          onChange={(v) => props.setValue(+v.target.value)}
        />
      )}

      <input
        type="number"
        name={props.name}
        value={props.value === 0 ? "" : props.value}
        min={props.min}
        max={props.max}
        placeholder="0"
        onChange={(v) => props.setValue(+v.target.value)}
      />
    </label>
  );
};

export default SettingsLabel;
