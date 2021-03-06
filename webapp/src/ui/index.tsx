import { Header } from "./header";
import { LandingPage } from "./landing";
import React from "react";

/** The actual application UI */
export const App: React.FC = () => {
  return (
    <div>
      <Header />
      <LandingPage />
    </div>
  );
};
