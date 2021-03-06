import React from "react";
import landingPicture from "./landing.jpg";

/** The landing page */
export const LandingPage: React.FC = () => {
  /* eslint-disable jsx-a11y/alt-text */
  return (
    <div className="row">
      <div className="col-12 col-lg-3 order-lg-3">Stuff Here</div>
      <div className="col-12 col-lg-9">
        <img
          src={landingPicture}
          role="presentation"
          className="img-fluid img-thumbnail rounded shadow"
        />
      </div>
    </div>
  );
};
