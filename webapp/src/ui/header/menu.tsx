import { Login } from "./login";
import React from "react";
import { UserMenu } from "./userMenu";
import { useAuth0 } from "@auth0/auth0-react";

/**
 * Wrapper around the menu area to show either the login button or the user menu.
 */
export const Menu: React.FC = () => {
  const { isAuthenticated } = useAuth0();

  if (isAuthenticated) {
    return <UserMenu />;
  } else {
    return <Login />;
  }
};
