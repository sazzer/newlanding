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
      <a
        className="nav-link dropdown-toggle"
        href="#"
        id="navbarDropdown"
        role="button"
        data-bs-toggle="dropdown"
        aria-expanded="false"
      >
        {user?.name}
      </a>
      <ul
        className="dropdown-menu dropdown-menu-lg-end dropdown-menu-dark"
        aria-labelledby="navbarDropdown"
      >
        <li>
          <a
            className="dropdown-item"
            href="#"
            onClick={() => logout({ returnTo: window.location.origin })}
          >
            {t("header.logout")}
          </a>
        </li>
      </ul>
    </li>
  );
};
