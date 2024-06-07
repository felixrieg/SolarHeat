import React, { ReactNode } from "react";
import "./styles/WidgetContainer.scss";

interface WidgetContainerProps {
  children: ReactNode;
}

const WidgetContainer = (props: WidgetContainerProps) => {
  return <div className="widgetContainer">{props.children}</div>;
};

export default WidgetContainer;
