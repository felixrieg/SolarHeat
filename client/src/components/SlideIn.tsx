import { ReactNode } from "react";
import "./styles/SlideIn.scss";

type Props = {
  show: boolean;
  left?: boolean;
  onClose: () => void;
  children: ReactNode;
};

const SlideIn = (props: Props) => {
  return (
    <div
      className={
        "SlideInContainer " +
        (props.show ? "show " : "hidden ") +
        (!!props.left ? "left" : "")
      }
    >
      <div className="ContentContainer">
        <div className="SlideInClose clickable" onClick={props.onClose}>
          X
        </div>
        <div className="SlideInContent scrollable">{props.children}</div>
      </div>
    </div>
  );
};

export default SlideIn;
