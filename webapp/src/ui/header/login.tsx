import React from "react";
import { useAuth0 } from "@auth0/auth0-react";
import { useTranslation } from "react-i18next";

/**
 * Header link to log the user in
 */
export const Login: React.FC = () => {
  const { loginWithRedirect } = useAuth0();
  const { t } = useTranslation();

  return (
    <li className="nav-item">
      <button
        className="nav-link btn btn-link"
        onClick={() => loginWithRedirect()}
      >
        {t("header.login")}
      </button>
    </li>
  );
};
