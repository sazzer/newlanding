import { Header } from "./header";
import React from "react";
import { useTranslation } from "react-i18next";

/** The actual application UI */
export const App: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div>
      <Header />
      <div>{t("hello")}</div>
    </div>
  );
};
