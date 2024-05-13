import { ReactNode, useEffect } from "react";
import "./styles/SlideIn.scss";

type Props = {
  show: boolean;
  left?: boolean;
  onClose: () => void;
  children: ReactNode;
};

const SlideIn = (props: Props) => {
  useEffect(() => {
    if (props.show) {
      document.body.style.backgroundColor = "#1F2041";
    } else {
      document.body.style.backgroundColor = "#E9D2C0";
    }
  }, [props.show]);

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
        <div className="SlideInContent">{props.children}</div>
      </div>
    </div>
  );
};

export default SlideIn;
