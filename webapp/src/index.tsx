import "./i18n";

import { App } from "./ui";
import { Auth0Provider } from "@auth0/auth0-react";
import { BrowserRouter } from "react-router-dom";
import React from "react";
import ReactDOM from "react-dom";
import env from "@beam-australia/react-env";

ReactDOM.render(
  <React.StrictMode>
    <Auth0Provider
      domain={env("AUTH0_DOMAIN")}
      clientId={env("AUTH0_CLIENTID")}
      redirectUri={window.location.origin}
      audience={env("AUTH0_AUDIENCE")}
      useRefreshTokens={true}
    >
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </Auth0Provider>
  </React.StrictMode>,
  document.getElementById("root")
);
