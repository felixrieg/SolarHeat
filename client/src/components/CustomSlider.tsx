import "./styles/CustomSlider.scss";

type Props = {
  left?: string;
  right?: string;
  showCurrent?: boolean;
  min?: number;
  max?: number;
  values?: string[];
  name?: string;
  label?: string;
  value: number;
  setValue: (value: number) => void;
  showCurrentRight?: boolean;
};

const CustomSlider = (props: Props) => {
  let min = props.values ? 0 : props.min;
  let max = props.values ? props.values.length - 1 : props.max;
  return (
    <div className="customSliderContainer">
      <div className="customSlider">
        {props.label && <div>{props.label}</div>}
        {props.showCurrent && (
          <div>
            {props.values ? props.values[props.value] ?? "" : props.value}
          </div>
        )}
      </div>
      <div className="customSlider">
        {props.left && <div className="left">{props.left}</div>}
        <input
          className="slider"
          type="range"
          name={props.name}
          value={props.value}
          min={min}
          max={max}
          onChange={(v) => props.setValue(+v.target.value)}
        />
        {props.right && !props.showCurrentRight && (
          <div className="right">{props.right}</div>
        )}
        {props.showCurrentRight && (
          <div className="right">
            {props.values ? props.values[props.value] ?? "" : props.value}
          </div>
        )}
      </div>
    </div>
  );
};

export default CustomSlider;
