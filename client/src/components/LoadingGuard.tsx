import React from "react";
import LoadingSpinner from "./LoadingSpinner";

interface LoadingGuardProps {
  isLoading: boolean;
  children: React.ReactNode;
}

const LoadingGuard = ({ isLoading, children }: LoadingGuardProps) => {
  return isLoading ? <LoadingSpinner /> : <>{children}</>;
};

export default LoadingGuard;
