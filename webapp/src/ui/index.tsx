import React from "react";
import { useTranslation } from "react-i18next";

export const App: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div>
      <div>{t("hello")}</div>
    </div>
  );
};
