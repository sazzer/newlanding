import React from "react";
import { useAuth0 } from "@auth0/auth0-react";
import { useTranslation } from "react-i18next";

/**
 * Header dropdown for the user menu.
 */
export const UserMenu: React.FC = () => {
  const { logout, user } = useAuth0();
  const { t } = useTranslation();

  return (
    <li className="nav-item dropdown">
      <button
        className="nav-link btn btn-link dropdown-toggle"
        id="navbarDropdown"
        data-bs-toggle="dropdown"
        aria-expanded="false"
      >
        {user?.name}
      </button>
      <ul
        className="dropdown-menu dropdown-menu-lg-end dropdown-menu-dark"
        aria-labelledby="navbarDropdown"
      >
        <li>
          <button
            className="dropdown-item btn btn-link"
            onClick={() => logout({ returnTo: window.location.origin })}
          >
            {t("header.logout")}
          </button>
        </li>
      </ul>
    </li>
  );
};
